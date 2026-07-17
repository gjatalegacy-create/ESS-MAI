// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  RRJEDHA — INTELIGJENCA E DËSHTIMEVE (kontratë ×3, byte-identike)        ║
// ║  GJATA LEGACY™ | Arkitekt: Bledar Gjata                                  ║
// ║                                                                          ║
// ║  PARIMI I ARKITEKTIT: «Sistemi nuk ka pse të dështojë. Kur një rrjedhë   ║
// ║  megjithatë ndodh, ajo duhet të marrë PËRCAKTIM (pse), SHËNIM (ku/kur)   ║
// ║  dhe KUFI (sa herë tolerohet) — pa penguar runtime-in.»                  ║
// ║                                                                          ║
// ║  Kjo kontratë NUK zëvendëson fail-closed — e PARAPRIN dhe e SQARON:      ║
// ║    1. PSE   — çdo rrjedhë klasifikohet me diagnozë (jo "error occurred") ║
// ║    2. SHËNIM— rreshti vuloset me CRC fnv1a64 (ledger-i s'gënjen veten)   ║
// ║    3. KUFI  — çdo klasë ka limit determinist; tejkalimi = AtLimit,       ║
// ║              vendimi mbetet te thirrësi sipas ligjit të tij (rrjedha     ║
// ║              KURRË s'e bllokon vetë verdiktin — zero pengesë runtime)    ║
// ║                                                                          ║
// ║  LIGJET E KËSAJ KONTRATE:                                                ║
// ║    • Zero varësi të jashtme (vetëm std)                                  ║
// ║    • Zero if/else — vetëm match                                          ║
// ║    • Shkrimi në disk është BEST-EFFORT: dështimi i vetë ledger-it        ║
// ║      raportohet me zë (eprintln) dhe kurrë s'ndal punën                  ║
// ║    • Numëruesit janë të kufizuar (u32 saturating) — zero rritje pa fund  ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use std::sync::{Mutex, OnceLock};
use super::pa_wire::fnv1a64;

/// Klasat e rrjedhave — çdo dështim i mundshëm ka EMËR dhe KUFI.
/// Kufiri = sa ndodhi tolerohen brenda një jete procesi para se thirrësi
/// të këshillohet të përshkallëzojë (AtLimit). Kufiri 0 = tolerancë zero.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RrjedhaClass {
    /// CRC i urës s'përputhet — bit i ndryshuar në transport (disk/RAM).
    TransportBitflip,
    /// Rresht i cunguar — shkrim i pjesshëm (crash i shkruesit / disk plot).
    TransportTruncated,
    /// Rresht legacy PAS një rreshti të vulosur — degradim i dyshimtë CRC.
    TransportDowngrade,
    /// Mutex i helmuar — një fill kreu panik duke mbajtur kyçin.
    LockPoisoned,
    /// Disk: leje e mohuar / I/O reale (JO skedar që mungon — ajo është normale).
    DiskDenied,
    /// Shkruesi tentoi të emetojë rresht që lexuesi do ta refuzonte —
    /// PARANDALUAR në burim (invarianti i telit u shkel para daljes).
    WireInvariant,
    /// Ankora GCL nuk u gjet aty ku ligji e priste (rrjedhë e brendshme logjike).
    AnchorLost,
    /// Tjetër — e paklasifikuar (duhet të mbetet e rrallë; auditohet).
    Other,
}

impl RrjedhaClass {
    /// Kodi tekstual i klasës — i qëndrueshëm për grep/audit.
    pub fn code(&self) -> &'static str {
        match self {
            RrjedhaClass::TransportBitflip   => "TRANSPORT_BITFLIP",
            RrjedhaClass::TransportTruncated => "TRANSPORT_TRUNCATED",
            RrjedhaClass::TransportDowngrade => "TRANSPORT_DOWNGRADE",
            RrjedhaClass::LockPoisoned       => "LOCK_POISONED",
            RrjedhaClass::DiskDenied         => "DISK_DENIED",
            RrjedhaClass::WireInvariant      => "WIRE_INVARIANT",
            RrjedhaClass::AnchorLost         => "ANCHOR_LOST",
            RrjedhaClass::Other              => "OTHER",
        }
    }

    /// KUFIRI determinist i klasës (ndodhi të toleruara para AtLimit).
    /// Vendosur nga ligji: degradimi CRC dhe shkelja e invariantit të telit
    /// kanë tolerancë ZERO — s'ka arsye legjitime që ato të ndodhin kurrë.
    pub fn limit(&self) -> u32 {
        match self {
            RrjedhaClass::TransportBitflip   => 3,
            RrjedhaClass::TransportTruncated => 3,
            RrjedhaClass::TransportDowngrade => 0,
            RrjedhaClass::LockPoisoned       => 1,
            RrjedhaClass::DiskDenied         => 2,
            RrjedhaClass::WireInvariant      => 0,
            RrjedhaClass::AnchorLost         => 1,
            RrjedhaClass::Other              => 5,
        }
    }

    /// Indeksi i numëruesit (i brendshëm).
    fn idx(&self) -> usize {
        match self {
            RrjedhaClass::TransportBitflip   => 0,
            RrjedhaClass::TransportTruncated => 1,
            RrjedhaClass::TransportDowngrade => 2,
            RrjedhaClass::LockPoisoned       => 3,
            RrjedhaClass::DiskDenied         => 4,
            RrjedhaClass::WireInvariant      => 5,
            RrjedhaClass::AnchorLost         => 6,
            RrjedhaClass::Other              => 7,
        }
    }
}

/// Verdikti i shënimit: brenda kufirit apo në/mbi kufi.
/// Vendimi ç'të bëhet mbetet te THIRRËSI — rrjedha vetëm informon.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RrjedhaVerdict {
    /// Ndodhia u shënua; numërimi (count) është brenda kufirit të klasës.
    Within(u32),
    /// Numërimi arriti/kaloi kufirin — thirrësi këshillohet të përshkallëzojë
    /// sipas ligjit të tij (p.sh. fail-closed të argumentuar). (count, limit)
    AtLimit(u32, u32),
}

/// Numëruesit globalë të procesit — të kufizuar, zero rritje memorie.
fn counters() -> &'static Mutex<[u32; 8]> {
    static C: OnceLock<Mutex<[u32; 8]>> = OnceLock::new();
    C.get_or_init(|| Mutex::new([0u32; 8]))
}

/// Pastrimi i diagnozës: '|' dhe rreshtat e rinj s'lejohen të infektojnë
/// formatin e vetë ledger-it (parandalim injektimi — ledger-i s'thyhet dot).
pub fn sanitize(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '|'  => '/',
            '\n' => ' ',
            '\r' => ' ',
            _    => c,
        })
        .collect()
}

/// Ndërton rreshtin e vulosur të ledger-it (vetë-CRC — ledger-i provon veten):
///   ts_ns|platform|site|class|count|diagnoza|c:%016x
pub fn entry_line(
    ts_ns:     u64,
    platform:  &'static str,
    site:      &'static str,
    class:     RrjedhaClass,
    count:     u32,
    diagnosis: &str,
) -> String {
    let body = format!(
        "{}|{}|{}|{}|{}|{}",
        ts_ns, platform, site, class.code(), count, sanitize(diagnosis)
    );
    let crc = fnv1a64(body.as_bytes());
    format!("{}|c:{:016x}\n", body, crc)
}

/// SHËNO një rrjedhë: numëro (i kufizuar), shkruaj në ledger (best-effort,
/// KURRË bllokuese), ktheje verdiktin ndaj kufirit.
///
/// `dir` = dosja handoff e platformës (thirrësi e jep — kontrata s'e di
/// mjedisin). Skedari: `<dir>/rrjedha_ledger.txt` (append, i përbashkët,
/// çdo rresht i vetë-vulosur me CRC).
pub fn note(
    dir:       &str,
    platform:  &'static str,
    site:      &'static str,
    class:     RrjedhaClass,
    diagnosis: &str,
) -> RrjedhaVerdict {
    // 1) Numëro — kyç i helmuar rikuperohet (ligji i njëjtë kudo: into_inner).
    let count = {
        let mut guard = match counters().lock() {
            Ok(g)         => g,
            Err(poisoned) => poisoned.into_inner(),
        };
        let i = class.idx();
        guard[i] = guard[i].saturating_add(1);
        guard[i]
    };

    // 2) Shkruaj — BEST-EFFORT: dështimi i ledger-it raportohet me zë,
    //    por kurrë s'e ndal punën (zero pengesë runtime).
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0);
    let line = entry_line(ts, platform, site, class, count, diagnosis);
    let path = format!("{}/rrjedha_ledger.txt", dir.trim_end_matches(['/', '\\']));
    use std::io::Write as _;
    match std::fs::OpenOptions::new().create(true).append(true).open(&path) {
        Ok(mut f) => match f.write_all(line.as_bytes()).and_then(|_| f.flush()) {
            Ok(())  => {}
            Err(e)  => eprintln!(
                "  [RRJEDHA] ALARM: ledger-i s'u shkrua ({e}) — shënimi humbi, puna vazhdon"),
        },
        Err(e) => eprintln!(
            "  [RRJEDHA] ALARM: {} s'u hap ({e}) — shënimi humbi, puna vazhdon", path),
    }

    // 3) Verdikti ndaj kufirit — informacion, jo urdhër.
    let lim = class.limit();
    match count > lim {
        true  => RrjedhaVerdict::AtLimit(count, lim),
        false => RrjedhaVerdict::Within(count),
    }
}

/// Lexim vetëm-audit i numëruesit të një klase (për teste/raporte).
pub fn class_count(class: RrjedhaClass) -> u32 {
    let guard = match counters().lock() {
        Ok(g)         => g,
        Err(poisoned) => poisoned.into_inner(),
    };
    guard[class.idx()]
}

// ════════════════════════════════════════════════════════════════════════════
// TESTE — kontrata provohet në SECILËN platformë (×3, byte-identike)
// ════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod rrjedha_tests {
    use super::*;
    use super::super::pa_wire::{verify_line, PaWireVerdict};

    #[test]
    fn sanitize_blocks_ledger_injection() {
        // Diagnoza me '|' dhe '\n' s'e thyen dot formatin e ledger-it.
        assert_eq!(sanitize("a|b\nc\rd"), "a/b c d");
    }

    #[test]
    fn entry_line_is_self_sealed_and_verifiable() {
        // Ledger-i ha ushqimin e vet: rreshti i tij kalon verify_line si
        // 7-fusha... jo — verify_line pret 3/5/6; ledger-i ka 7. Prandaj
        // vula provohet DREJTPËRDREJT: CRC mbi trupin para '|c:'.
        let line = entry_line(9, "SHADOW", "gateway::feed", RrjedhaClass::TransportBitflip, 1, "bit i kthyer");
        let trimmed = line.trim_end_matches('\n');
        let (body, crc_field) = trimmed.rsplit_once("|c:").expect("fusha c: e pranishme");
        let want = u64::from_str_radix(crc_field, 16).expect("hex i vlefshëm");
        assert_eq!(fnv1a64(body.as_bytes()), want, "vetë-CRC duhet të përputhet");
        // Dhe kontrata pa_wire e sheh 7-fushëshin si të papritur (ndarje e pastër
        // ledger↔urë — ledger-i s'ngatërrohet kurrë me ankorat).
        assert!(matches!(verify_line(trimmed), PaWireVerdict::Corrupt(_)));
    }

    #[test]
    fn zero_tolerance_classes_hit_limit_immediately() {
        // Degradimi dhe shkelja e invariantit kanë kufi 0 — ndodhi e parë = AtLimit.
        assert_eq!(RrjedhaClass::TransportDowngrade.limit(), 0);
        assert_eq!(RrjedhaClass::WireInvariant.limit(), 0);
        let d = std::env::temp_dir();
        let dir = d.to_str().expect("temp dir utf8");
        match note(dir, "TEST", "test::zero", RrjedhaClass::TransportDowngrade, "provë kufiri") {
            RrjedhaVerdict::AtLimit(c, 0) => assert!(c >= 1),
            other => panic!("pritej AtLimit(_,0), erdhi {:?}", other),
        }
    }

    #[test]
    fn counters_saturate_and_never_wrap() {
        // Numëruesi është saturating — zero mbështjellje, zero panik.
        let c0 = class_count(RrjedhaClass::Other);
        let d = std::env::temp_dir();
        let dir = d.to_str().expect("temp dir utf8");
        let _ = note(dir, "TEST", "test::sat", RrjedhaClass::Other, "njëshi");
        assert_eq!(class_count(RrjedhaClass::Other), c0.saturating_add(1));
    }
}
