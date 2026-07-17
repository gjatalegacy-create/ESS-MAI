// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  FFI_RING.RS — RING BUFFER repr(C) SPSC (zero-copy në kufirin FFI)    ║
// ║  Gjata Legacy™ | Bledar Gjata   ·   FAZA 3 (opsionale, e përforcuar)  ║
// ║                                                                          ║
// ║  Dorëzim C → Rust pa alokim për mesazh dhe pa Vec të ndërmjetëm.       ║
// ║  Një PRODHUES (C, p.sh. Light Hydrator) + një KONSUMATOR (Shadow).     ║
// ║                                                                          ║
// ║  LAYOUT repr(C) (i ndashëm me C përmes shared-memory):                 ║
// ║    offset 0:  head: u32   (konsumatori Rust e avancon)                ║
// ║    offset 4:  tail: u32   (prodhuesi C e avancon)                     ║
// ║    offset 8:  slots[RING_CAP] × RingSlot                              ║
// ║                                                                          ║
// ║  RENDITJA ATOMIKE: prodhuesi boton tail me Release; konsumatori e      ║
// ║  lexon me Acquire → happens-before mbi përmbajtjen e slot-it.         ║
// ║  SPSC → asnjë mutex; slot-i lexohet NË VEND (zero-copy).              ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use std::alloc::{alloc_zeroed, Layout};
use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicU32, Ordering};

/// Numri i slot-eve — DUHET fuqi e 2-shit (maskim me AND).
pub const RING_CAP: u32 = 64;
const MASK: u32 = RING_CAP - 1;

/// Kapaciteti i payload-it për slot (bytes).
pub const SLOT_BYTES: usize = 4096;

const _: () = assert!(RING_CAP.is_power_of_two(), "RING_CAP duhet fuqi e 2-shit");

/// Një slot i vetëm transporti — repr(C), POD (pa Drop).
#[repr(C)]
pub struct RingSlot {
    pub len:   u32,                 // bytes të vlefshëm në `bytes`
    pub flags: u32,                 // p.sh. primitive_flags (vula 500)
    pub seq:   u64,                 // numër sekuence (rendi global)
    pub bytes: [u8; SLOT_BYTES],
}

/// Unaza SPSC. repr(C) → e ndashme me C; UnsafeCell është transparent.
#[repr(C)]
pub struct ShadowRing {
    head:  AtomicU32,               // konsumatori (Rust)
    tail:  AtomicU32,               // prodhuesi (C ose Rust)
    slots: UnsafeCell<[RingSlot; RING_CAP as usize]>,
}

// SPSC: një prodhues + një konsumator. Ndarja mes dy fijeve është e sigurt
// sepse secila prek vetëm "anën" e vet, e sinkronizuar me head/tail atomikë.
// (Send merret automatikisht; vetëm Sync duhet mbivendosur — UnsafeCell e mohon.)
unsafe impl Sync for ShadowRing {}

impl ShadowRing {
    /// Alokon unazën të zeruar DREJT në heap (pa kopje të madhe në stack).
    pub fn new() -> Box<Self> {
        let layout = Layout::new::<ShadowRing>();
        // SAFETY: ShadowRing është POD (atomics + array u8); gjendja krejt-zero
        // është e vlefshme (head=tail=0, slots=0). Box e çliron me të njëjtin layout.
        unsafe {
            let ptr = alloc_zeroed(layout) as *mut ShadowRing;
            assert!(!ptr.is_null(), "alloc_zeroed dështoi për ShadowRing");
            Box::from_raw(ptr)
        }
    }

    #[inline]
    pub fn capacity(&self) -> u32 { RING_CAP }

    /// Numri i mesazheve të papërpunuara.
    #[inline]
    pub fn len(&self) -> u32 {
        let tail = self.tail.load(Ordering::Acquire);
        let head = self.head.load(Ordering::Acquire);
        tail.wrapping_sub(head)
    }

    #[inline]
    pub fn is_empty(&self) -> bool { self.len() == 0 }

    #[inline]
    pub fn is_full(&self) -> bool { self.len() >= RING_CAP }

    /// PRODHUESI: shkruan një mesazh. `false` nëse unaza është plot ose payload-i
    /// e kalon `SLOT_BYTES` (cungohet jo — refuzohet, që të mos humbasë integriteti).
    pub fn try_produce(&self, data: &[u8], flags: u32) -> bool {
        match data.len() > SLOT_BYTES {
            true => return false,
            false => {}
        }
        let tail = self.tail.load(Ordering::Relaxed); // vetëm prodhuesi e shkruan
        let head = self.head.load(Ordering::Acquire);
        match tail.wrapping_sub(head) >= RING_CAP {
            true => return false, // plot
            false => {}
        }

        // SAFETY: SPSC — vetëm prodhuesi prek slot[tail], dhe konsumatori asnjëherë
        // të njëjtin slot njëkohësisht (head≠tail kur unaza s'është bosh/plot).
        // Marrim pointer te NJË slot (jo referencë te gjithë array-i) → pa aliasing.
        let slot = unsafe { &mut *(self.slots.get() as *mut RingSlot).add((tail & MASK) as usize) };
        let n = data.len();
        slot.len = n as u32;
        slot.flags = flags;
        slot.seq = tail as u64;
        slot.bytes[..n].copy_from_slice(data);

        self.tail.store(tail.wrapping_add(1), Ordering::Release); // boton
        true
    }

    /// KONSUMATORI: lexon mesazhin më të vjetër NË VEND (zero-copy) dhe ia kalon
    /// `f` si `(flags, seq, &[u8])`. `None` nëse unaza është bosh. Slot-i mund të
    /// ripërdoret vetëm PASI `f` të kthehet (head avancohet në fund).
    pub fn consume_with<R>(&self, f: impl FnOnce(u32, u64, &[u8]) -> R) -> Option<R> {
        let head = self.head.load(Ordering::Relaxed); // vetëm konsumatori e shkruan
        let tail = self.tail.load(Ordering::Acquire);
        match head == tail {
            true => return None, // bosh
            false => {}
        }

        // SAFETY: SPSC — slot[head] u botua nga prodhuesi (Release→Acquire mësipër);
        // prodhuesi s'e mbishkruan derisa `head` të avancojë poshtë. Pointer te NJË
        // slot (jo referencë te gjithë array-i) → pa aliasing me prodhuesin.
        let slot = unsafe { &*(self.slots.get() as *const RingSlot).add((head & MASK) as usize) };
        let n = (slot.len as usize).min(SLOT_BYTES);
        let view = &slot.bytes[..n]; // ZERO-COPY: pa kopje, pa Vec
        let out = f(slot.flags, slot.seq, view);

        self.head.store(head.wrapping_add(1), Ordering::Release); // liron slot-in
        Some(out)
    }

    /// Lehtësi: konsumon një mesazh duke e kopjuar në `(flags, Vec<u8>)`.
    pub fn try_consume(&self) -> Option<(u32, Vec<u8>)> {
        self.consume_with(|flags, _seq, data| (flags, data.to_vec()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn produce_then_consume_fifo() {
        let ring = ShadowRing::new();
        assert!(ring.is_empty());
        assert!(ring.try_produce(b"alpha", 0xA451));
        assert!(ring.try_produce(b"beta", 0x0000));
        assert_eq!(ring.len(), 2);

        let (f0, d0) = ring.try_consume().unwrap();
        assert_eq!(f0, 0xA451);
        assert_eq!(d0, b"alpha");
        let (f1, d1) = ring.try_consume().unwrap();
        assert_eq!(f1, 0x0000);
        assert_eq!(d1, b"beta");
        assert!(ring.try_consume().is_none());
        assert!(ring.is_empty());
    }

    #[test]
    fn zero_copy_view_sees_payload_and_seq() {
        let ring = ShadowRing::new();
        ring.try_produce(b"first", 1);
        ring.try_produce(b"second", 2);
        let seen = ring
            .consume_with(|flags, seq, data| (flags, seq, data.len()))
            .unwrap();
        assert_eq!(seen, (1, 0, 5)); // seq i parë = 0
    }

    #[test]
    fn full_ring_refuses_then_drains() {
        let ring = ShadowRing::new();
        for i in 0..RING_CAP {
            assert!(ring.try_produce(&[i as u8], i), "duhej pranuar #{i}");
        }
        assert!(ring.is_full());
        assert!(!ring.try_produce(b"overflow", 99)); // plot → refuzon
        // Konsumo një → vend i lirë → pranon sërish.
        assert!(ring.try_consume().is_some());
        assert!(ring.try_produce(b"x", 99));
    }

    #[test]
    fn oversized_payload_is_refused() {
        let ring = ShadowRing::new();
        let big = vec![0u8; SLOT_BYTES + 1];
        assert!(!ring.try_produce(&big, 0)); // > SLOT_BYTES → refuzohet
        assert!(ring.is_empty());
    }

    #[test]
    fn wraparound_preserves_order() {
        let ring = ShadowRing::new();
        // Mbush, zbraz, dhe kalo kufirin disa herë.
        for round in 0..4u32 {
            for i in 0..RING_CAP {
                assert!(ring.try_produce(&[(round + i) as u8], round * 100 + i));
            }
            for i in 0..RING_CAP {
                let (flags, _) = ring.try_consume().unwrap();
                assert_eq!(flags, round * 100 + i);
            }
        }
        assert!(ring.is_empty());
    }
}
