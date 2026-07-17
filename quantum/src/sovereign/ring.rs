// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  SOVEREIGN/RING.RS — RingBuffer 0-COPY + EpistemicVector              ║
// ║  GJATA LEGACY™ — Quantum Platform                                     ║
// ║                                                                          ║
// ║  Adaptim i RingBuffer + EpistemicVector (nga ligji 0-copy).           ║
// ║                                                                          ║
// ║  eBPF/Hardware → RingBuffer → LgcBridge → EpistemicVector → LIM       ║
// ║                                                                          ║
// ║  MATEMATIKA: head = (head+1) & MASK → modulo pa degëzim, O(1).        ║
// ║  EpistemicVector.mass = dot8(axes, W) → produkt skalar linear.        ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use std::cell::{Cell, UnsafeCell};
use std::marker::PhantomData;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use crate::sovereign::laws::dot8;
use crate::sovereign::lgc_gate::{LgcGate, SovErr};

// ─────────────────────────────────────────────────────────────────────────────
// KONSTANTET — N = fuqi e 2, MASK = N−1
// ─────────────────────────────────────────────────────────────────────────────

/// Numri i slot-eve (fuqi e 2 → mask zëvendëson modulo).
pub const RING_N: usize = 64;
/// mask = N−1: head & MASK == head % N, zero branch.
pub const RING_MASK: usize = RING_N - 1;

// ─────────────────────────────────────────────────────────────────────────────
// EBPF PACKET — 8 dimensionet që vijnë nga hidratimi
// ─────────────────────────────────────────────────────────────────────────────

/// Paketa nga eBPF/hardware me 8 dimensionet e EpistemicVector + CRC.
#[derive(Debug, Clone, Copy)]
pub struct EbpfPacket {
    pub d1: f32, pub d2: f32, pub d3: f32, pub d4: f32,
    pub d5: f32, pub d6: f32, pub d7: f32, pub d8: f32,
    pub crc: u32,
}

impl EbpfPacket {
    pub fn zeroed() -> Self {
        Self { d1:0.0,d2:0.0,d3:0.0,d4:0.0,d5:0.0,d6:0.0,d7:0.0,d8:0.0, crc:0 }
    }

    pub fn from_axes(axes: [f32; 8]) -> Self {
        let mut p = Self {
            d1:axes[0],d2:axes[1],d3:axes[2],d4:axes[3],
            d5:axes[4],d6:axes[5],d7:axes[6],d8:axes[7], crc:0,
        };
        p.crc = p.compute_crc();
        p
    }

    /// CRC32 i thjeshtë mbi 8 floats (integritet transporti).
    pub fn compute_crc(&self) -> u32 {
        let vals = [self.d1,self.d2,self.d3,self.d4,self.d5,self.d6,self.d7,self.d8];
        let mut crc: u32 = 0xFFFF_FFFF;
        for v in vals {
            let bytes = v.to_bits().to_le_bytes();
            for b in bytes {
                crc ^= b as u32;
                let mut k = 0;
                while k < 8 {
                    let mask = (crc & 1).wrapping_neg();
                    crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
                    k += 1;
                }
            }
        }
        !crc
    }

    pub fn verify_crc(&self) -> bool {
        self.compute_crc() == self.crc
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// RING SLOT — UnsafeCell për mutim të brendshëm pa &mut
// ─────────────────────────────────────────────────────────────────────────────

struct RingSlot {
    data: UnsafeCell<EbpfPacket>,
}

struct RingCore {
    slots:   Vec<RingSlot>,
    head:    AtomicUsize,  // producer
    tail:    AtomicUsize,  // consumer
    written: AtomicU64,
    dropped: AtomicU64,
}

// RingCore ndahet vetëm nga çifti unik RingProducer/RingConsumer.
// SAFETY: konstruktorët e endpoint-eve janë privatë; `split(self)` prodhon
// saktësisht një producer dhe një consumer. Producer publikon `head` me
// Release pasi shkruan slot-in; consumer e lexon me Acquire. Consumer publikon
// `tail` me Release pasi lexon; producer e lexon me Acquire para ripërdorimit.
unsafe impl Sync for RingCore {}

impl RingCore {
    fn new() -> Self {
        let mut slots = Vec::with_capacity(RING_N);
        for _ in 0..RING_N {
            slots.push(RingSlot {
                data: UnsafeCell::new(EbpfPacket::zeroed()),
            });
        }
        Self {
            slots,
            head:    AtomicUsize::new(0),
            tail:    AtomicUsize::new(0),
            written: AtomicU64::new(0),
            dropped: AtomicU64::new(0),
        }
    }

    fn push(&self, pkt: EbpfPacket) -> Result<(), SovErr> {
        let head = self.head.load(Ordering::Relaxed);
        let tail = self.tail.load(Ordering::Acquire);

        // Full check pa branch: (head − tail) >= N. Match në vend të if.
        match head.wrapping_sub(tail) >= RING_N {
            true => {
                self.dropped.fetch_add(1, Ordering::Relaxed);
                return Err(SovErr::NotFound);
            }
            false => {}
        }

        let pos = head & RING_MASK;
        // SAFETY: endpoint-i unik producer është i vetmi shkrues; slot-i nuk
        // ripërdoret para se consumer të publikojë tail me Release.
        unsafe {
            *self.slots[pos].data.get() = pkt;
        }
        self.head.store(head.wrapping_add(1), Ordering::Release);
        self.written.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    fn pop(&self) -> Option<EbpfPacket> {
        let tail = self.tail.load(Ordering::Relaxed);
        let head = self.head.load(Ordering::Acquire);

        match tail == head {
            true => return None,
            false => {}
        }

        let pos = tail & RING_MASK;
        // SAFETY: endpoint-i unik consumer është i vetmi lexues; Acquire mbi
        // head sheh shkrimin e publikuar nga producer para Release-it.
        let pkt = unsafe { *self.slots[pos].data.get() };
        self.tail.store(tail.wrapping_add(1), Ordering::Release);
        Some(pkt)
    }

    fn written(&self) -> u64 { self.written.load(Ordering::Relaxed) }
    fn dropped(&self) -> u64 { self.dropped.load(Ordering::Relaxed) }
}

// ─────────────────────────────────────────────────────────────────────────────
// RING BUFFER — pronar sekuencial + ndarje e tipizuar SPSC
// ─────────────────────────────────────────────────────────────────────────────

/// Pronari fillestar i ring-ut. Nuk është `Sync`: safe Rust nuk mund ta ndajë
/// drejtpërdrejt mes shumë thread-eve. Për SPSC paralel përdoret `split(self)`,
/// i cili prodhon saktësisht një endpoint producer dhe një endpoint consumer.
pub struct RingBuffer {
    core: Arc<RingCore>,
    _single_owner: PhantomData<Cell<()>>, // !Sync, por Send
}

/// Endpoint-i unik i prodhuesit. Nuk është Clone/Sync; `push` kërkon `&mut self`.
pub struct RingProducer {
    core: Arc<RingCore>,
    _not_sync: PhantomData<Cell<()>>,
}

/// Endpoint-i unik i konsumatorit. Nuk është Clone/Sync; `pop` kërkon `&mut self`.
pub struct RingConsumer {
    core: Arc<RingCore>,
    _not_sync: PhantomData<Cell<()>>,
}

impl RingBuffer {
    pub fn new() -> Self {
        Self {
            core: Arc::new(RingCore::new()),
            _single_owner: PhantomData,
        }
    }

    /// Ndarje e vetme, e konsumueshme: një producer + një consumer.
    /// Kjo e bën disiplinën SPSC pronë të tipit, jo vetëm koment.
    pub fn split(self) -> (RingProducer, RingConsumer) {
        let RingBuffer { core, .. } = self;
        let producer = RingProducer {
            core: Arc::clone(&core),
            _not_sync: PhantomData,
        };
        let consumer = RingConsumer {
            core,
            _not_sync: PhantomData,
        };
        (producer, consumer)
    }

    /// Rruga sekuenciale ekzistuese; e sigurt sepse `RingBuffer` nuk është Sync.
    pub fn push(&self, pkt: EbpfPacket) -> Result<(), SovErr> { self.core.push(pkt) }
    pub fn pop(&self) -> Option<EbpfPacket> { self.core.pop() }
    pub fn written(&self) -> u64 { self.core.written() }
    pub fn dropped(&self) -> u64 { self.core.dropped() }
}

impl RingProducer {
    pub fn push(&mut self, pkt: EbpfPacket) -> Result<(), SovErr> {
        self.core.push(pkt)
    }
    pub fn written(&self) -> u64 { self.core.written() }
    pub fn dropped(&self) -> u64 { self.core.dropped() }
}

impl RingConsumer {
    pub fn pop(&mut self) -> Option<EbpfPacket> { self.core.pop() }
    pub fn written(&self) -> u64 { self.core.written() }
    pub fn dropped(&self) -> u64 { self.core.dropped() }
}

impl Default for RingBuffer {
    fn default() -> Self { Self::new() }
}

// ─────────────────────────────────────────────────────────────────────────────
// EPISTEMIC VECTOR — output i hidratimit zero-copy
// ─────────────────────────────────────────────────────────────────────────────

/// 8 akset epistemike — vektori që ushqen LIM-in.
#[derive(Debug, Clone, Copy)]
pub struct Ev {
    pub axes: [f32; 8],
}

impl Ev {
    /// Peshat e masës (identike me frymën e LIM epistemic_mass).
    pub const W: [f32; 8] = [0.18, 0.15, 0.12, 0.15, 0.15, 0.10, 0.10, 0.05];

    /// Masa = dot8(axes, W) — produkt skalar linear.
    pub fn mass(&self) -> f32 {
        dot8(&self.axes, &Self::W)
    }

    /// Ndërton Ev nga paketa (zero kopjim shtesë).
    pub fn from_packet(p: &EbpfPacket) -> Self {
        Self { axes: [p.d1,p.d2,p.d3,p.d4,p.d5,p.d6,p.d7,p.d8] }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// LGC BRIDGE — handshake C → Rust: verifikon CRC + seal → prodhon Ev
// ─────────────────────────────────────────────────────────────────────────────

/// Ura që lexon nga ringu, verifikon integritetin, dhe prodhon EpistemicVector.
/// LAW_1: asnjë Ev nuk kalon pa verifikim (CRC + seal token).
pub struct LgcBridge {
    gate: LgcGate,
}

impl LgcBridge {
    pub fn new(expected_seal: u64) -> Self {
        Self { gate: LgcGate::new(expected_seal) }
    }

    /// Merr paketën e radhës nga pronari sekuencial, e verifikon dhe prodhon Ev.
    pub fn receive(&self, ring: &RingBuffer, runtime_seal: u64) -> Result<Ev, SovErr> {
        let pkt = ring.pop().ok_or(SovErr::NotFound)?;
        self.verify_and_project(pkt, runtime_seal)
    }

    /// Merr paketën nga endpoint-i unik consumer i rrjedhës paralele SPSC.
    /// E njëjta portë CRC + seal zbatohet; ndarja e thread-eve nuk krijon
    /// rrugë alternative që anashkalon autorizimin epistemik.
    pub fn receive_from(
        &self,
        consumer: &mut RingConsumer,
        runtime_seal: u64,
    ) -> Result<Ev, SovErr> {
        let pkt = consumer.pop().ok_or(SovErr::NotFound)?;
        self.verify_and_project(pkt, runtime_seal)
    }

    /// Rruga e vetme e transformimit packet → Ev për të dy format e pronësisë.
    fn verify_and_project(&self, pkt: EbpfPacket, runtime_seal: u64) -> Result<Ev, SovErr> {
        // 1. Verifiko CRC (integritet transporti). Match total në vend të if.
        match pkt.verify_crc() {
            false => return Err(SovErr::NotFound),
            true => {}
        }

        // 2. Verifiko seal-in — token-i prodhohet dhe konsumohet brenda portës.
        let _token = self.gate.open(runtime_seal)?;

        // 3. Vetëm paketa e verifikuar projektohet në EpistemicVector.
        Ok(Ev::from_packet(&pkt))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sovereign::laws::SEAL_EBPF;

    #[test]
    fn ring_push_pop_fifo() {
        let ring = RingBuffer::new();
        let p1 = EbpfPacket::from_axes([0.1; 8]);
        let p2 = EbpfPacket::from_axes([0.9; 8]);
        ring.push(p1).unwrap();
        ring.push(p2).unwrap();
        let out1 = ring.pop().unwrap();
        let out2 = ring.pop().unwrap();
        assert!((out1.d1 - 0.1).abs() < 1e-6);
        assert!((out2.d1 - 0.9).abs() < 1e-6);
        assert!(ring.pop().is_none()); // bosh
    }

    #[test]
    fn typed_spsc_split_preserves_fifo() {
        fn assert_send<T: Send>() {}
        assert_send::<RingProducer>();
        assert_send::<RingConsumer>();

        let (mut producer, mut consumer) = RingBuffer::new().split();
        producer.push(EbpfPacket::from_axes([0.2; 8])).unwrap();
        producer.push(EbpfPacket::from_axes([0.8; 8])).unwrap();
        assert!((consumer.pop().unwrap().d1 - 0.2).abs() < 1e-6);
        assert!((consumer.pop().unwrap().d1 - 0.8).abs() < 1e-6);
        assert!(consumer.pop().is_none());
    }

    #[test]
    fn typed_spsc_consumer_passes_through_lgc_bridge() {
        let (mut producer, mut consumer) = RingBuffer::new().split();
        producer.push(EbpfPacket::from_axes([0.6; 8])).unwrap();
        let bridge = LgcBridge::new(SEAL_EBPF);
        let ev = bridge.receive_from(&mut consumer, SEAL_EBPF).unwrap();
        assert!((ev.axes[0] - 0.6).abs() < 1e-6);
    }

    #[test]
    fn ring_full_drops() {
        let ring = RingBuffer::new();
        // Mbush ringun (RING_N pakete).
        for _ in 0..RING_N {
            ring.push(EbpfPacket::from_axes([0.5; 8])).unwrap();
        }
        // E (N+1)-ta → drop.
        assert!(ring.push(EbpfPacket::from_axes([0.5; 8])).is_err());
        assert_eq!(ring.dropped(), 1);
    }

    #[test]
    fn crc_detects_corruption() {
        let mut p = EbpfPacket::from_axes([0.5; 8]);
        assert!(p.verify_crc());
        p.d1 = 0.99; // korruptim pa rillogaritur CRC
        assert!(!p.verify_crc());
    }

    #[test]
    fn ev_mass_is_dot_product() {
        let ev = Ev { axes: [1.0; 8] };
        let expected: f32 = Ev::W.iter().sum();
        assert!((ev.mass() - expected).abs() < 1e-5);
    }

    #[test]
    fn lgc_bridge_full_handshake() {
        let ring = RingBuffer::new();
        ring.push(EbpfPacket::from_axes([0.7; 8])).unwrap();
        let bridge = LgcBridge::new(SEAL_EBPF);
        // Seal i saktë → Ev prodhohet.
        let ev = bridge.receive(&ring, SEAL_EBPF).unwrap();
        assert!((ev.axes[0] - 0.7).abs() < 1e-6);
    }

    #[test]
    fn lgc_bridge_rejects_bad_seal() {
        let ring = RingBuffer::new();
        ring.push(EbpfPacket::from_axes([0.7; 8])).unwrap();
        let bridge = LgcBridge::new(SEAL_EBPF);
        // Seal i gabuar → SealMismatch.
        assert!(bridge.receive(&ring, 0xDEAD).is_err());
    }
}
