// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  VAULT_DISK.RS — PERSISTENCA SOVRANE (Write-Ahead Log) · FAZA 2  ║
// ║  Gjata Legacy™  |  Arkitekt: Bledar Gjata                             ║
// ║                                                                          ║
// ║  Memoria sovrane duhet të MBIJETOJË restart-in. Ky modul ofron një     ║
// ║  backend disku append-only (WAL) për KnowledgeVault.                  ║
// ║                                                                          ║
// ║  PARIMI — WAL LOGJIK + RIPËRSËRITJE BESNIKE:                          ║
// ║    Regjistrojmë THIRRJET logjike (Primitive/Verified/Negative/Confirm) ║
// ║    në renditje. Në startup, ripërsëriten përmes TË NJËJTËS logjikë —   ║
// ║    dedup-i negativ dhe promovimi Primitive→Legacy riprodhohen EKZAKT.  ║
// ║    (Një primitiv fresh s'e arrin kurrë pragun 0.75 → çdo promovim      ║
// ║     vjen pas një `Confirm` të regjistruar.)                            ║
// ║                                                                          ║
// ║  DURABILITET: çdo rekord = [LEN][CRC32][PAYLOAD], pastaj fsync.        ║
// ║  SIGURI NDAJ RRËZIMIT: bishti i dëmtuar/i cunguar shpërfillet dhe      ║
// ║  log-u shkurtohet te rekordi i fundit i vlefshëm.                     ║
// ║                                                                          ║
// ║  ZERO varësi të jashtme — vetëm std. Codec binar i shkruar me dorë.   ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::knowledge_vault::VaultBackend;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

// ── Identifikuesit e skedarit/rekordit ──────────────────────────────────────
const FILE_MAGIC:  u64 = 0x474A_5356_4C30_3401; // "GJSVL04" + version 1
const HEADER_LEN:  usize = 12;                   // 8 magic + 4 version
const FILE_VERSION: u32 = 2;

// Etiketat e ngjarjeve
const TAG_PRIMITIVE: u8 = 1;
const TAG_VERIFIED:  u8 = 2;
const TAG_NEGATIVE:  u8 = 3;
const TAG_CONFIRM:   u8 = 4;

// ════════════════════════════════════════════════════════════════════════════
// §1  CRC32 (IEEE, pa tabelë) — integritet rekordi
// ════════════════════════════════════════════════════════════════════════════

fn crc32(data: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFF_FFFF;
    for &b in data {
        crc ^= b as u32;
        let mut i = 0;
        while i < 8 {
            let mask = (crc & 1).wrapping_neg();
            crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
            i += 1;
        }
    }
    !crc
}

// ════════════════════════════════════════════════════════════════════════════
// §2  ENKODIM — shkrues binar mbi Vec<u8> (little-endian)
// ════════════════════════════════════════════════════════════════════════════

fn put_u8(b: &mut Vec<u8>, v: u8)  { b.push(v); }
fn put_u32(b: &mut Vec<u8>, v: u32) { b.extend_from_slice(&v.to_le_bytes()); }
fn put_u64(b: &mut Vec<u8>, v: u64) { b.extend_from_slice(&v.to_le_bytes()); }
fn put_f32(b: &mut Vec<u8>, v: f32) { b.extend_from_slice(&v.to_le_bytes()); }
fn put_bytes(b: &mut Vec<u8>, s: &[u8]) { put_u32(b, s.len() as u32); b.extend_from_slice(s); }
fn put_str(b: &mut Vec<u8>, s: &str) { put_bytes(b, s.as_bytes()); }

// ════════════════════════════════════════════════════════════════════════════
// §3  DEKODIM — lexues me kursor; kthen None nëse mungojnë bytes
// ════════════════════════════════════════════════════════════════════════════

struct Cursor<'a> { data: &'a [u8], pos: usize }

impl<'a> Cursor<'a> {
    fn new(data: &'a [u8]) -> Self { Self { data, pos: 0 } }
    fn remaining(&self) -> usize { self.data.len() - self.pos }

    fn take(&mut self, n: usize) -> Option<&'a [u8]> {
        match self.remaining() < n { true => return None, false => {} }
        let s = &self.data[self.pos..self.pos + n];
        self.pos += n;
        Some(s)
    }
    fn u8(&mut self)  -> Option<u8>  { self.take(1).map(|s| s[0]) }
    fn u32(&mut self) -> Option<u32> { self.take(4).and_then(|s| match s.try_into() { Ok(a) => Some(u32::from_le_bytes(a)), Err(_) => None /* CORRUPT_U32 */ }) }
    fn u64(&mut self) -> Option<u64> { self.take(8).and_then(|s| match s.try_into() { Ok(a) => Some(u64::from_le_bytes(a)), Err(_) => None /* CORRUPT_U64 */ }) }
    fn f32(&mut self) -> Option<f32> { self.take(4).and_then(|s| match s.try_into() { Ok(a) => Some(f32::from_le_bytes(a)), Err(_) => None /* CORRUPT_F32 */ }) }
    fn bytes(&mut self) -> Option<Vec<u8>> {
        let n = self.u32()? as usize;
        // P8.1: WAL guard — 64MB kufizim OOM. Zero if — match.
        let within_limit = n <= 64 * 1024 * 1024;
        match within_limit {
            true  => self.take(n).map(|s| s.to_vec()),
            false => None,  // tejkalim → refuzo (mbron nga OOM)
        }
    }
    fn string(&mut self) -> Option<String> {
        let raw = self.bytes()?;
        String::from_utf8(raw).ok()
    }
}

// ════════════════════════════════════════════════════════════════════════════
// §4  REPLAY OP — ngjarja e dekoduar për ripërsëritje në startup
// ════════════════════════════════════════════════════════════════════════════

/// Ngjarjet e regjistruara (versioni i zotëruar, për ripërsëritje).
#[derive(Debug, Clone)]
pub enum ReplayOp {
    Primitive { input_id: u64, vector_points: f32, temporal_score: f32, mass: f32, seal: u32, path: u8, ts: u64, domain: String, bytes: Vec<u8> },
    Verified  { input_id: u64, band: u8, mass: f32, ts: u64, bytes: Vec<u8> },
    Negative  { input_id: u64, reason: u32, process: String, mass: f32, frequency: u32, ts: u64, bytes: Vec<u8> },
    Confirm   { input_id: u64, hits: u32 },
}

/// Dekodon një payload të vetëm (pas verifikimit CRC) → ReplayOp.
fn decode_payload(payload: &[u8]) -> Option<ReplayOp> {
    let mut c = Cursor::new(payload);
    let tag = c.u8()?;
    match tag {
        TAG_PRIMITIVE => Some(ReplayOp::Primitive {
            input_id:       c.u64()?,
            vector_points:  c.f32()?,
            temporal_score: c.f32()?,
            mass:           c.f32()?,
            seal:           c.u32()?,
            path:           c.u8()?,
            ts:             c.u64()?,
            domain:         c.string()?,
            bytes:          c.bytes()?,
        }),
        TAG_VERIFIED => Some(ReplayOp::Verified {
            input_id: c.u64()?,
            band:     c.u8()?,
            mass:     c.f32()?,
            ts:       c.u64()?,
            bytes:    c.bytes()?,
        }),
        TAG_NEGATIVE => Some(ReplayOp::Negative {
            input_id:  c.u64()?,
            reason:    c.u32()?,
            process:   c.string()?,
            mass:      c.f32()?,
            frequency: c.u32()?,
            ts:        c.u64()?,
            bytes:     c.bytes()?,
        }),
        TAG_CONFIRM => Some(ReplayOp::Confirm {
            input_id: c.u64()?,
            hits:     c.u32()?,
        }),
        _ => None, /* etiketë e panjohur → ndal (siguri) */
    }
}

// ════════════════════════════════════════════════════════════════════════════
// §5  DISK BACKEND — WAL append-only me fsync
// ════════════════════════════════════════════════════════════════════════════

#[derive(Debug)]
pub struct DiskBackend {
    file: Mutex<File>,
    path: PathBuf,
}

impl DiskBackend {
    /// Hap (ose krijon) log-un. Kthen backend-in + ngjarjet ekzistuese për
    /// ripërsëritje. Bishti i dëmtuar shpërfillet dhe log-u shkurtohet.
    pub fn open(path: &Path) -> io::Result<(Self, Vec<ReplayOp>)> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false) // ruaj log-un; set_len(good_len) riparon vetëm bishtin e dëmtuar
            .open(path)?;

        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;

        let (ops, good_len) = match buf.is_empty() {
            true => {
                // Skedar i ri → shkruaj header-in.
                let mut header = Vec::with_capacity(HEADER_LEN);
                put_u64(&mut header, FILE_MAGIC);
                put_u32(&mut header, FILE_VERSION);
                file.seek(SeekFrom::Start(0))?;
                file.write_all(&header)?;
                file.sync_all()?;
                (Vec::new(), HEADER_LEN as u64)
            }
            false => Self::parse(&buf)?,
        };

        // Shkurto bishtin e dëmtuar (nëse ka) dhe poziciono te fundi i mirë.
        file.set_len(good_len)?;
        file.seek(SeekFrom::Start(good_len))?;

        Ok((Self { file: Mutex::new(file), path: path.to_path_buf() }, ops))
    }

    /// Parse i plotë i një buffer-i ekzistues. Kthen (ops, offset_i_mirë).
    fn parse(buf: &[u8]) -> io::Result<(Vec<ReplayOp>, u64)> {
        match buf.len() < HEADER_LEN {
            true => return Err(io::Error::new(io::ErrorKind::InvalidData, "header i cunguar")),
            false => {}
        }
        let magic = match buf[0..8].try_into() {
            Ok(a)  => u64::from_le_bytes(a),
            Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidData,
                "CORRUPT_HEADER: magic i palexueshëm")),
        };
        let version = match buf[8..12].try_into() {
            Ok(a)  => u32::from_le_bytes(a),
            Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidData,
                "CORRUPT_HEADER: version i palexueshëm")),
        };
        match magic != FILE_MAGIC {
            true => return Err(io::Error::new(io::ErrorKind::InvalidData, "magic i panjohur (skedar i huaj)")),
            false => {}
        }
        match version != FILE_VERSION {
            true => return Err(io::Error::new(io::ErrorKind::InvalidData, "version i papërputhshëm")),
            false => {}
        }

        let mut ops = Vec::new();
        let mut pos = HEADER_LEN;
        let mut good = HEADER_LEN as u64;

        // Çdo rekord: [LEN u32][CRC u32][PAYLOAD]. Ndal te dëmtimi/cungimi i parë.
        loop {
            match buf.len() - pos < 8 { true => break, false => {} }
            // Guard-i 'len-pos<8' garanton 4 bytes; Err strukturalisht i pamundur —
            // por ligji: korrupsioni NDALET i emërtuar, kurrë s'bëhet 0 e butë.
            let len = match buf[pos..pos + 4].try_into() {
                Ok(a)  => u32::from_le_bytes(a) as usize,
                Err(_) => break, /* CORRUPT_LEN → tail truncation te 'good' */
            };
            let crc = match buf[pos + 4..pos + 8].try_into() {
                Ok(a)  => u32::from_le_bytes(a),
                Err(_) => break, /* CORRUPT_CRC → tail truncation te 'good' */
            };
            let body_start = pos + 8;
            match buf.len() - body_start < len { true => break, false => {} } /* tail i cunguar */
            let payload = &buf[body_start..body_start + len];
            match crc32(payload) != crc { true => break, false => {} } /* tail i dëmtuar */
            match decode_payload(payload) {
                Some(op) => ops.push(op),
                None => break,                                     /* etiketë e keqe → ndal */
            }
            pos = body_start + len;
            good = pos as u64;
        }
        Ok((ops, good))
    }

    /// Korniza + CRC + shkrim + fsync (durabilitet i fortë).
    fn write_record(&self, payload: &[u8]) {
        let mut frame = Vec::with_capacity(8 + payload.len());
        put_u32(&mut frame, payload.len() as u32);
        put_u32(&mut frame, crc32(payload));
        frame.extend_from_slice(payload);

        match self.file.lock() {
            Ok(mut f) => {
                // FAIL-LOUD (ULTIMATUM): WAL-i i vault-it = durabiliteti sovran.
                // I/O dështimi raportohet (RAM mbetet burimi i leximit, por
                // humbja e persistencës DUHET audituar), s'fshihet me `let _`.
                match f.write_all(&frame).and_then(|_| f.flush()).and_then(|_| f.sync_all()) {
                    Ok(())  => {}
                    Err(e)  => eprintln!("[VAULT_DISK] ALARM: WAL write dështoi ({e}) — durabiliteti i rrezikuar (RAM mbetet aktiv)"),
                }
            }
            Err(_) => eprintln!("[VAULT_DISK] ALARM: file lock i helmuar — WAL frame s'u shkrua (durabilitet i rrezikuar)"),
        }
    }

    pub fn path(&self) -> &Path { &self.path }
}

// ── Implementimi i grepit të persistencës ──────────────────────────────────
impl VaultBackend for DiskBackend {
    #[allow(clippy::too_many_arguments)]
    fn on_primitive(&self, input_id: u64, vector_points: f32, temporal_score: f32, mass: f32, seal: u32, path: u8, domain: &str, bytes: &[u8], ts: u64) {
        let mut p = Vec::with_capacity(48 + domain.len() + bytes.len());
        put_u8(&mut p, TAG_PRIMITIVE);
        put_u64(&mut p, input_id);
        put_f32(&mut p, vector_points);
        put_f32(&mut p, temporal_score);
        put_f32(&mut p, mass);
        put_u32(&mut p, seal);
        put_u8(&mut p, path);
        put_u64(&mut p, ts);
        put_str(&mut p, domain);
        put_bytes(&mut p, bytes);
        self.write_record(&p);
    }

    fn on_verified(&self, input_id: u64, band: u8, mass: f32, bytes: &[u8], ts: u64) {
        let mut p = Vec::with_capacity(24 + bytes.len());
        put_u8(&mut p, TAG_VERIFIED);
        put_u64(&mut p, input_id);
        put_u8(&mut p, band);
        put_f32(&mut p, mass);
        put_u64(&mut p, ts);
        put_bytes(&mut p, bytes);
        self.write_record(&p);
    }

    fn on_negative(&self, input_id: u64, reason: u32, process: &str, mass: f32, frequency: u32, bytes: &[u8], ts: u64) {
        let mut p = Vec::with_capacity(32 + process.len() + bytes.len());
        put_u8(&mut p, TAG_NEGATIVE);
        put_u64(&mut p, input_id);
        put_u32(&mut p, reason);
        put_str(&mut p, process);
        put_f32(&mut p, mass);
        put_u32(&mut p, frequency);
        put_u64(&mut p, ts);
        put_bytes(&mut p, bytes);
        self.write_record(&p);
    }

    fn on_confirm(&self, input_id: u64, hits: u32) {
        let mut p = Vec::with_capacity(16);
        put_u8(&mut p, TAG_CONFIRM);
        put_u64(&mut p, input_id);
        put_u32(&mut p, hits);
        self.write_record(&p);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crc32_known_vector() {
        // CRC32(IEEE) i "123456789" = 0xCBF43926
        assert_eq!(crc32(b"123456789"), 0xCBF4_3926);
    }

    #[test]
    fn roundtrip_encode_decode_primitive() {
        let mut p = Vec::new();
        put_u8(&mut p, TAG_PRIMITIVE);
        put_u64(&mut p, 42);
        put_f32(&mut p, 0.9);   // vector_points
        put_f32(&mut p, 0.8);   // temporal_score
        put_f32(&mut p, 0.95);  // mass
        put_u32(&mut p, 0xA451); // seal
        put_u8(&mut p, 1);      // path (Standard)
        put_u64(&mut p, 1000);  // ts
        put_str(&mut p, "science"); // domain
        put_bytes(&mut p, b"abc");
        match decode_payload(&p).expect("roundtrip_encode_decode_primitive: rezultat i papritur në test") {
            ReplayOp::Primitive { input_id, vector_points, temporal_score, mass, seal, path, ts, domain, bytes } => {
                assert_eq!(input_id, 42);
                assert!((vector_points - 0.9).abs() < 1e-6);
                assert!((temporal_score - 0.8).abs() < 1e-6);
                assert!((mass - 0.95).abs() < 1e-6);
                assert_eq!(seal, 0xA451);
                assert_eq!(path, 1);
                assert_eq!(ts, 1000);
                assert_eq!(domain, "science");
                assert_eq!(bytes, b"abc");
            }
            _ => panic!("tag i gabuar"),
        }
    }

    #[test]
    fn truncated_payload_returns_none() {
        let mut p = Vec::new();
        put_u8(&mut p, TAG_PRIMITIVE);
        put_u64(&mut p, 42);
        // mungojnë fushat e tjera
        assert!(decode_payload(&p).is_none());
    }
}
