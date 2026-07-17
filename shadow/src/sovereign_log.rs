// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  SOVEREIGN_LOG.RS — WAL GJENERIK append-only (durabilitet sovran)     ║
// ║  Gjata Legacy™ | Bledar Gjata                                        ║
// ║                                                                          ║
// ║  Përgjithësim besnik i `vault_disk` për kujtesat sovrane (APUPK, SNB). ║
// ║  Korniza identike: header [u64 MAGIC][u32 VERSION]; rekord              ║
// ║  [u32 LEN][u32 CRC][PAYLOAD]. CRC32 IEEE (poly 0xEDB88320, pa tabelë). ║
// ║                                                                          ║
// ║  Politika I/O (si vault): write_record best-effort + fsync; bishti i   ║
// ║  dëmtuar/cunguar shpërfillet dhe log-u shkurtohet në hapje. RAM mbetet ║
// ║  burimi i leximit; disku është durabiliteti.                          ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

const HEADER_LEN: usize = 12; // u64 magic + u32 version

/// CRC32 IEEE (poly 0xEDB88320), pa tabelë — identik me vault_disk.
pub fn crc32(data: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFF_FFFF;
    for &b in data {
        crc ^= b as u32;
        for _ in 0..8 {
            let mask = (crc & 1).wrapping_neg();
            crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
        }
    }
    !crc
}

// ── KODUES rekordi (Little-Endian) ──────────────────────────────────────────

#[derive(Default)]
pub struct RecWriter {
    buf: Vec<u8>,
}

impl RecWriter {
    pub fn new() -> Self {
        Self { buf: Vec::new() }
    }
    pub fn u8(&mut self, v: u8) -> &mut Self {
        self.buf.push(v);
        self
    }
    pub fn u32(&mut self, v: u32) -> &mut Self {
        self.buf.extend_from_slice(&v.to_le_bytes());
        self
    }
    pub fn u64(&mut self, v: u64) -> &mut Self {
        self.buf.extend_from_slice(&v.to_le_bytes());
        self
    }
    pub fn f32(&mut self, v: f32) -> &mut Self {
        self.buf.extend_from_slice(&v.to_le_bytes());
        self
    }
    pub fn bytes(&mut self, s: &[u8]) -> &mut Self {
        self.u32(s.len() as u32);
        self.buf.extend_from_slice(s);
        self
    }
    pub fn str(&mut self, s: &str) -> &mut Self {
        self.bytes(s.as_bytes())
    }
    /// Listë stringjesh: [u32 count][str…] (p.sh. flow_trace e SNB).
    pub fn str_list(&mut self, items: &[String]) -> &mut Self {
        self.u32(items.len() as u32);
        for s in items {
            self.str(s);
        }
        self
    }
    pub fn finish(self) -> Vec<u8> {
        self.buf
    }
}

// ── DEKODUES rekordi ────────────────────────────────────────────────────────

pub struct RecReader<'a> {
    data: &'a [u8],
    pos:  usize,
}

impl<'a> RecReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }
    fn take(&mut self, n: usize) -> Option<&'a [u8]> {
        let end = self.pos.checked_add(n)?;
        let s = self.data.get(self.pos..end)?;
        self.pos = end;
        Some(s)
    }
    pub fn u8(&mut self) -> Option<u8> {
        self.take(1).map(|s| s[0])
    }
    pub fn u32(&mut self) -> Option<u32> {
        self.take(4).and_then(|s| match s.try_into() { Ok(a) => Some(u32::from_le_bytes(a)), Err(_) => None /* CORRUPT_U32 */ })
    }
    pub fn u64(&mut self) -> Option<u64> {
        self.take(8).and_then(|s| match s.try_into() { Ok(a) => Some(u64::from_le_bytes(a)), Err(_) => None /* CORRUPT_U64 */ })
    }
    pub fn f32(&mut self) -> Option<f32> {
        self.take(4).and_then(|s| match s.try_into() { Ok(a) => Some(f32::from_le_bytes(a)), Err(_) => None /* CORRUPT_F32 */ })
    }
    pub fn bytes(&mut self) -> Option<Vec<u8>> {
        let len = self.u32()? as usize;
        self.take(len).map(|s| s.to_vec())
    }
    pub fn string(&mut self) -> Option<String> {
        let raw = self.bytes()?;
        String::from_utf8(raw).ok()
    }
    /// Listë stringjesh: [u32 count][str…].
    pub fn str_list(&mut self) -> Option<Vec<String>> {
        let count = self.u32()? as usize;
        let mut out = Vec::with_capacity(count);
        for _ in 0..count {
            out.push(self.string()?);
        }
        Some(out)
    }
}

// ── LOG-u sovran ────────────────────────────────────────────────────────────

/// Log append-only me header MAGIC/VERSION të personalizueshme (që një skedar i
/// huaj të mos lexohet gabimisht). Kthen payload-et e papërpunuara në hapje.
#[derive(Debug)]
pub struct SovereignLog {
    file:    Mutex<File>,
    magic:   u64,
    version: u32,
    _path:   PathBuf,
}

impl SovereignLog {
    /// Hap (ose krijon) log-un. Kthen log-un + payload-et ekzistuese (të dekoduara
    /// nga thirrësi). Bishti i dëmtuar shpërfillet dhe skedari shkurtohet.
    pub fn open(path: &Path, magic: u64, version: u32) -> io::Result<(Self, Vec<Vec<u8>>)> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false) // ruaj log-un; set_len(good_len) riparon vetëm bishtin e dëmtuar
            .open(path)?;

        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;

        let (payloads, good_len) = match buf.is_empty() {
            true => {
                let mut header = Vec::with_capacity(HEADER_LEN);
                header.extend_from_slice(&magic.to_le_bytes());
                header.extend_from_slice(&version.to_le_bytes());
                file.seek(SeekFrom::Start(0))?;
                file.write_all(&header)?;
                file.sync_all()?;
                (Vec::new(), HEADER_LEN as u64)
            }
            false => Self::parse(&buf, magic, version)?,
        };

        file.set_len(good_len)?;
        file.seek(SeekFrom::Start(good_len))?;

        Ok((
            Self { file: Mutex::new(file), magic, version, _path: path.to_path_buf() },
            payloads,
        ))
    }

    fn parse(buf: &[u8], magic: u64, version: u32) -> io::Result<(Vec<Vec<u8>>, u64)> {
        match buf.len() < HEADER_LEN {
            true => return Err(io::Error::new(io::ErrorKind::InvalidData, "header i cunguar")),
            false => {}
        }
        let m = match buf[0..8].try_into() {
            Ok(a)  => u64::from_le_bytes(a),
            Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidData,
                "CORRUPT_HEADER: magic i palexueshëm")),
        };
        let v = match buf[8..12].try_into() {
            Ok(a)  => u32::from_le_bytes(a),
            Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidData,
                "CORRUPT_HEADER: version i palexueshëm")),
        };
        match m != magic {
            true => return Err(io::Error::new(io::ErrorKind::InvalidData, "magic i panjohur (skedar i huaj)")),
            false => {}
        }
        match v != version {
            true => return Err(io::Error::new(io::ErrorKind::InvalidData, "version i papërputhshëm")),
            false => {}
        }

        let mut payloads = Vec::new();
        let mut pos = HEADER_LEN;
        let mut good = HEADER_LEN as u64;

        // Çdo rekord: [LEN u32][CRC u32][PAYLOAD]. Ndal te dëmtimi/cungimi i parë.
        loop {
            match buf.len() - pos < 8 {
                true => break,
                false => {}
            }
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
            match buf.len() - body_start < len {
                true => break, // tail i cunguar
                false => {}
            }
            let payload = &buf[body_start..body_start + len];
            match crc32(payload) != crc {
                true => break, // tail i dëmtuar
                false => {}
            }
            payloads.push(payload.to_vec());
            pos = body_start + len;
            good = pos as u64;
        }
        Ok((payloads, good))
    }

    /// Korniza + CRC + shkrim + flush + fsync, me rezultat të kontrollueshëm.
    /// Përdoret nga rrjedhat ku një dëshmitar sovran NUK lejohet të lindë pa
    /// durabilitet të provuar (p.sh. APUPK ProjectContextWitness).
    pub fn append_checked(&self, payload: &[u8]) -> io::Result<()> {
        let mut frame = Vec::with_capacity(8 + payload.len());
        frame.extend_from_slice(&(payload.len() as u32).to_le_bytes());
        frame.extend_from_slice(&crc32(payload).to_le_bytes());
        frame.extend_from_slice(payload);

        let mut f = self.file.lock().map_err(|_| {
            io::Error::new(io::ErrorKind::Other, "file lock i SovereignLog është helmuar")
        })?;
        f.write_all(&frame)?;
        f.flush()?;
        f.sync_all()
    }

    /// Variant legacy best-effort për kujtesat që lejojnë degradim në RAM.
    /// Rrjedhat fail-closed duhet të përdorin `append_checked` drejtpërdrejt.
    pub fn append(&self, payload: &[u8]) {
        match self.append_checked(payload) {
            Ok(()) => {}
            Err(e) => eprintln!("[SOVEREIGN_LOG] ALARM: shkrimi i ledger-it dështoi ({e}) — integriteti i auditit i rrezikuar"),
        }
    }

    pub fn magic(&self) -> u64 {
        self.magic
    }
    pub fn version(&self) -> u32 {
        self.version
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp(tag: &str) -> PathBuf {
        let mut p = std::env::temp_dir();
        let ns = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("temp: rezultat i papritur në test")
            .as_nanos();
        p.push(format!("sovlog_{tag}_{ns}.log"));
        p
    }

    #[test]
    fn crc32_known_vector() {
        // CRC32 i "123456789" = 0xCBF43926 (vektor standard IEEE).
        assert_eq!(crc32(b"123456789"), 0xCBF4_3926);
    }

    #[test]
    fn roundtrip_records_survive_reopen() {
        let path = temp("rt");
        let magic = 0x1122_3344_5566_7788;
        {
            let (log, existing) = SovereignLog::open(&path, magic, 1).expect("roundtrip_records_survive_reopen: rezultat i papritur në test");
            assert!(existing.is_empty());
            let mut w = RecWriter::new();
            w.u64(42).str("alpha").str_list(&["x".to_string(), "y".to_string()]);
            log.append(&w.finish());
        }
        {
            let (_log, payloads) = SovereignLog::open(&path, magic, 1).expect("roundtrip_records_survive_reopen: rezultat i papritur në test");
            assert_eq!(payloads.len(), 1);
            let mut r = RecReader::new(&payloads[0]);
            assert_eq!(r.u64(), Some(42));
            assert_eq!(r.string(), Some("alpha".to_string()));
            assert_eq!(r.str_list(), Some(vec!["x".to_string(), "y".to_string()]));
        }
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn foreign_magic_is_rejected() {
        let path = temp("foreign");
        {
            let (log, _) = SovereignLog::open(&path, 0xAAAA, 1).expect("foreign_magic_is_rejected: rezultat i papritur në test");
            log.append(b"data");
        }
        // Hapje me magic tjetër → gabim (skedar i huaj).
        assert!(SovereignLog::open(&path, 0xBBBB, 1).is_err());
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn corrupt_tail_is_truncated() {
        let path = temp("corrupt");
        let magic = 0xDEAD_BEEF_0000_0001;
        {
            let (log, _) = SovereignLog::open(&path, magic, 1).expect("corrupt_tail_is_truncated: rezultat i papritur në test");
            let mut w = RecWriter::new();
            w.u32(7);
            log.append(&w.finish());
        }
        // Shto bytes mbeturina (rekord i paplotë).
        {
            use std::io::Write;
            let mut f = OpenOptions::new().append(true).open(&path).expect("corrupt_tail_is_truncated: rezultat i papritur në test");
            f.write_all(&[0xFF, 0xFF, 0xFF]).expect("corrupt_tail_is_truncated: rezultat i papritur në test");
        }
        {
            let (_log, payloads) = SovereignLog::open(&path, magic, 1).expect("corrupt_tail_is_truncated: rezultat i papritur në test");
            assert_eq!(payloads.len(), 1); // rekordi i mirë mbijeton; bishti hidhet
        }
        let _ = std::fs::remove_file(&path);
    }
}
