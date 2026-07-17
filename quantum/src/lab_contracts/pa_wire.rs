// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  PA_WIRE — KONTRATA E TRANSPORTIT TË PRIMITIVE ANCHOR (PACP në urë)      ║
// ║  GJATA LEGACY™ | Arkitekt: Bledar Gjata                                  ║
// ║                                                                          ║
// ║  PRIMITIVE ANCHOR CONTINUITY PRINCIPLE (PACP) — AKSIOMA:                 ║
// ║  «Primitive Anchor (i₀) duhet të mbetet i pandryshuar gjatë gjithë      ║
// ║   ciklit të ekzekutimit dhe të shërbejë si referenca përfundimtare e    ║
// ║   verifikimit. Asnjë konkluzion nuk mund të konsiderohet i verifikuar   ║
// ║   pa u krahasuar me Primitive Anchor.»                                  ║
// ║                                                                          ║
// ║  Kjo kontratë e bën PACP-in TË PROVUESHËM në transport: ankora që Light ║
// ║  shkruan është BYTE-PËR-BYTE ankora që Shadow lexon — CRC (FNV-1a 64,   ║
// ║  u64, zero varësi të jashtme) mbi TË PESTA fushat e plota. Çdo bit i    ║
// ║  ndryshuar në urë = TransportCorrupt = kollaps refuzimi para gjykimit.  ║
// ║                                                                          ║
// ║  FORMATI v-fushë (pas-pajtueshmëri e deklaruar):                        ║
// ║    3 fusha: sid|pa_hex16|ts                    (legacy, split bosh)      ║
// ║    5 fusha: sid|pa_hex16|ts|xi:…|yi:…          (legacy, pa CRC)          ║
// ║    6 fusha: sid|pa_hex16|ts|xi:…|yi:…|c:%016x  (PACP — CRC i detyruar)   ║
// ║  CRC llogaritet mbi bajtet e "sid|pa_hex16|ts|xi:…|yi:…" EKZAKTËSISHT   ║
// ║  siç shkruhen (pa '\n', pa fushën c:). Zero if/else — vetëm match.       ║
// ╚══════════════════════════════════════════════════════════════════════════╝

/// FNV-1a 64-bit — i njëjti algoritëm në të TRE platformat (kontratë e gdhendur).
/// Vektor i njohur: fnv1a64(b"a") == 0xaf63_dc4c_8601_ec8c.
pub fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in bytes.iter() {
        h ^= *b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

/// Verdikti i verifikimit të një rreshti PA në anën lexuese (Shadow).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PaWireVerdict {
    /// 6 fusha, CRC i saktë — PACP i provuar byte-për-byte në urë.
    SealedOk,
    /// 3 ose 5 fusha (formati i vjetër, pa CRC) — pranohet i DEKLARUAR.
    LegacyOk,
    /// KORRUPSION: CRC nuk përputhet / prefiks c: i keqformuar / hex i keq.
    Corrupt(&'static str),
}

/// Trupi kanonik i rreshtit (5 fushat pa CRC) — burimi i vetëm i së vërtetës
/// për llogaritjen e CRC-së në TË DYJA anët e urës.
pub fn canonical_body(sid: &str, pa_hex16: &str, ts_ns: u64, xi_wire: &str, yi_wire: &str) -> String {
    format!("{}|{}|{}|{}|{}", sid, pa_hex16, ts_ns, xi_wire, yi_wire)
}

/// ANA SHKRUESE (Light): ndërton rreshtin e plotë 6-fushësh me CRC + '\n'.
/// `split_wire` është dalja e PrimitiveSplit::to_wire() → "xi:…|yi:…".
pub fn encode_line(sid: &str, pa_hex16: &str, ts_ns: u64, split_wire: &str) -> String {
    let body = format!("{}|{}|{}|{}", sid, pa_hex16, ts_ns, split_wire);
    let crc  = fnv1a64(body.as_bytes());
    format!("{}|c:{:016x}\n", body, crc)
}

/// ANA LEXUESE (Shadow): gjykon integritetin e një rreshti të plotë (pa '\n').
/// NUK e interpreton përmbajtjen (kjo mbetet te gateway) — vetëm PACP-në e urës.
pub fn verify_line(line: &str) -> PaWireVerdict {
    let parts: Vec<&str> = line.split('|').collect();
    match parts.as_slice() {
        // Legacy — pranohen të deklaruara (pa provë CRC).
        [_sid, _hex, _ts]                      => PaWireVerdict::LegacyOk,
        [_sid, _hex, _ts, _xi, _yi]            => PaWireVerdict::LegacyOk,
        // PACP — 6 fusha: CRC i detyruar mbi 5 të parat, byte-për-byte.
        [sid, hex, ts, xi, yi, crc_field] => {
            let tail = match crc_field.strip_prefix("c:") {
                Some(t) => t,
                None    => return PaWireVerdict::Corrupt("fusha e 6-të pa prefiks c:"),
            };
            let want = match u64::from_str_radix(tail.trim(), 16) {
                Ok(v)  => v,
                Err(_) => return PaWireVerdict::Corrupt("CRC hex i keqformuar"),
            };
            let body = format!("{}|{}|{}|{}|{}", sid, hex, ts, xi, yi);
            let have = fnv1a64(body.as_bytes());
            match have == want {
                true  => PaWireVerdict::SealedOk,
                false => PaWireVerdict::Corrupt("CRC nuk përputhet — ankora u ndryshua në urë"),
            }
        }
        _ => PaWireVerdict::Corrupt("numër fushash i papritur"),
    }
}

/// PARANDALIMI NË BURIM (v1.3.2): shkruesi NUK lejohet të emetojë rresht që
/// lexuesi do ta refuzonte. Rreshti ndërtohet, pastaj VETË-VERIFIKOHET me
/// të njëjtin `verify_line` që përdor Shadow — nëse s'del `SealedOk`
/// (p.sh. term me '|' që zhvendos fushat, '\n' i brendshëm që copëton
/// rreshtin), kthimi është `Err` me arsyen dhe ASGJË s'del në tel.
///
/// FILOZOFIA E ARKITEKTIT: «Sistemi nuk ka pse të dështojë» — dështimi i
/// mundshëm i lexuesit shuhet KËTU, para se të lindë. Fail-closed te Shadow
/// mbetet si mur i fundit; ky funksion e bën murin të paprekshëm nga vetja.
pub fn encode_line_verified(
    sid:        &str,
    pa_hex16:   &str,
    ts_ns:      u64,
    split_wire: &str,
) -> Result<String, &'static str> {
    let line    = encode_line(sid, pa_hex16, ts_ns, split_wire);
    let trimmed = line.trim_end_matches('\n');
    // '\n' i brendshëm do ta copëtonte rreshtin te `.lines()` i lexuesit.
    match trimmed.contains('\n') {
        true  => return Err("invariant i telit: '\\n' brenda trupit — rreshti do të copëtohej"),
        false => {}
    }
    // I njëjti gjykatës si Shadow: fushë e zhvendosur nga '|' në terma,
    // hex i keq, çdo devijim — kapet PARA daljes.
    match verify_line(trimmed) {
        PaWireVerdict::SealedOk => Ok(line),
        PaWireVerdict::LegacyOk => Err("invariant i telit: dalja s'është 6-fushëshe e vulosur"),
        PaWireVerdict::Corrupt(_) => Err("invariant i telit: fushat u zhvendosën ('|' në terma?) — rreshti refuzohet në burim"),
    }
}

/// ══ SEAL GJENERIK (v1.4.0) — E NJËJTA VULË PËR ÇDO URË TEKSTUALE ═══════════
/// PA mbetet te encode_line/verify_line (3/5/6 — kontratë e kyçur). Urat e
/// tjera (PD 6/7→8, të ardhshmet) marrin TË NJËJTIN ligj '|c:%016x' përmes
/// këtyre tre funksioneve — një vulë, një gjykatës, zero divergjencë.

/// Vulos një trup të çfarëdoshëm '|'-të-ndarë: CRC fnv1a64 mbi bajtet EKZAKTE.
pub fn seal_body(body: &str) -> String {
    let crc = fnv1a64(body.as_bytes());
    format!("{}|c:{:016x}\n", body, crc)
}

/// Gjykatësi gjenerik: numri i fushave ∈ `legacy_fields` → LegacyOk i
/// deklaruar; == `sealed_fields` → CRC i detyruar mbi (n-1) fushat e para;
/// çdo numër tjetër → Corrupt. Zero if/else — vetëm match.
pub fn verify_line_generic(
    line:          &str,
    legacy_fields: &[usize],
    sealed_fields: usize,
) -> PaWireVerdict {
    let parts: Vec<&str> = line.split('|').collect();
    let n = parts.len();
    match n == sealed_fields {
        true => {
            let tail = match parts[n - 1].strip_prefix("c:") {
                Some(t) => t,
                None    => return PaWireVerdict::Corrupt("fusha e fundit pa prefiks c:"),
            };
            let want = match u64::from_str_radix(tail.trim(), 16) {
                Ok(v)  => v,
                Err(_) => return PaWireVerdict::Corrupt("CRC hex i keqformuar"),
            };
            let body = parts[..n - 1].join("|");
            match fnv1a64(body.as_bytes()) == want {
                true  => PaWireVerdict::SealedOk,
                false => PaWireVerdict::Corrupt("CRC nuk përputhet — rreshti u ndryshua në urë"),
            }
        }
        false => match legacy_fields.contains(&n) {
            true  => PaWireVerdict::LegacyOk,
            false => PaWireVerdict::Corrupt("numër fushash i papritur"),
        },
    }
}

/// Shkruesi gjenerik i VETË-GJYKUAR (parandalimi në burim, ligji i v1.3.2):
/// trupi vuloset dhe rreshti gjykohet me gjykatësin e lexuesit PARA daljes —
/// '\n' i brendshëm ose fushë e prishur → Err, asgjë s'del në tel.
pub fn seal_body_verified(
    body:          &str,
    legacy_fields: &[usize],
) -> Result<String, &'static str> {
    match body.contains('\n') {
        true  => return Err("invariant i telit: '\\n' brenda trupit — rreshti do të copëtohej"),
        false => {}
    }
    let line = seal_body(body);
    let sealed_fields = body.split('|').count() + 1;
    match verify_line_generic(line.trim_end_matches('\n'), legacy_fields, sealed_fields) {
        PaWireVerdict::SealedOk => Ok(line),
        PaWireVerdict::LegacyOk => Err("invariant i telit: dalja s'u njoh si e vulosur"),
        PaWireVerdict::Corrupt(_) => Err("invariant i telit: vula gjenerike s'u vetë-verifikua"),
    }
}

// ════════════════════════════════════════════════════════════════════════════
// TESTE — kontrata provohet në SECILËN platformë (×3, byte-identike)
// ════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod pa_wire_tests {
    use super::*;

    #[test]
    fn fnv_known_vector_locks_algorithm() {
        // Vektori i njohur FNV-1a 64 — algoritmi i kyçur ndër-platformë.
        assert_eq!(fnv1a64(b"a"), 0xaf63_dc4c_8601_ec8c);
    }

    #[test]
    fn encode_then_verify_roundtrip_is_sealed() {
        // PACP roundtrip: ajo që shkruhet është ekzaktësisht ajo që verifikohet.
        let line = encode_line("SES_P", "00000000000000aa", 123456789, "xi:flamur,kuq|yi:shqiponje");
        let trimmed = line.trim_end_matches('\n');
        assert_eq!(verify_line(trimmed), PaWireVerdict::SealedOk);
    }

    #[test]
    fn single_bit_flip_is_corrupt() {
        // PACP: çdo bit i ndryshuar në urë kapet — 'kuq' → 'kur'.
        let line = encode_line("SES_P", "00000000000000aa", 123456789, "xi:flamur,kuq|yi:shqiponje");
        let tampered = line.trim_end_matches('\n').replace("kuq", "kur");
        match verify_line(&tampered) {
            PaWireVerdict::Corrupt(m) => assert!(m.contains("përputhet")),
            other => panic!("pritej Corrupt(CRC), erdhi {:?}", other),
        }
    }

    #[test]
    fn legacy_three_and_five_fields_declared_ok() {
        assert_eq!(verify_line("S1|00000000000000ff|9"), PaWireVerdict::LegacyOk);
        assert_eq!(verify_line("S1|00000000000000ff|9|xi:a|yi:b"), PaWireVerdict::LegacyOk);
    }

    #[test]
    fn bad_crc_prefix_and_bad_hex_are_corrupt() {
        match verify_line("S1|aa|9|xi:a|yi:b|x:0011223344556677") {
            PaWireVerdict::Corrupt(m) => assert!(m.contains("prefiks")),
            other => panic!("pritej Corrupt(prefiks), erdhi {:?}", other),
        }
        match verify_line("S1|aa|9|xi:a|yi:b|c:JOHEX") {
            PaWireVerdict::Corrupt(m) => assert!(m.contains("hex")),
            other => panic!("pritej Corrupt(hex), erdhi {:?}", other),
        }
    }

    #[test]
    fn encode_verified_clean_terms_are_sealed() {
        // Parandalimi në burim: terma të pastër → rresht i vulosur, gati për tel.
        let line = encode_line_verified("SES_V", "00000000000000aa", 7, "xi:flamur,kuq|yi:shqiponje")
            .expect("terma të pastër duhet të vulosen");
        assert_eq!(verify_line(line.trim_end_matches('\n')), PaWireVerdict::SealedOk);
    }

    #[test]
    fn encode_verified_refuses_pipe_and_newline_in_terms() {
        // '|' në term zhvendos fushat; '\n' copëton rreshtin — TË DYJA
        // shuhen NË BURIM: asgjë e refuzueshme s'del kurrë në tel.
        match encode_line_verified("SES_V", "00000000000000aa", 7, "xi:fla|mur|yi:shqiponje") {
            Err(m) => assert!(m.contains("zhvendos") || m.contains("6-fushëshe")),
            Ok(l)  => panic!("term me '|' duhej refuzuar në burim, doli: {l}"),
        }
        match encode_line_verified("SES_V", "00000000000000aa", 7, "xi:fla\nmur|yi:shqiponje") {
            Err(m) => assert!(m.contains("copëtohej")),
            Ok(l)  => panic!("term me '\\n' duhej refuzuar në burim, doli: {l}"),
        }
    }

    #[test]
    fn generic_seal_pd_roundtrip_7_to_8_is_sealed() {
        // URA 1 (PD): trupi 7-fushësh → rreshti 8-fushësh i vulosur → SealedOk.
        let body = "SES_PD|koncepti|0.9123|0.8456|0.7000|4|pyetja_e_rradhes";
        let line = seal_body_verified(body, &[6, 7]).expect("trup i pastër duhet të vuloset");
        assert_eq!(verify_line_generic(line.trim_end_matches('\n'), &[6, 7], 8),
            PaWireVerdict::SealedOk);
    }

    #[test]
    fn generic_seal_single_bit_flip_is_corrupt() {
        let body = "SES_PD|koncepti|0.9123|0.8456|0.7000|4|hint";
        let line = seal_body(body);
        let tampered = line.trim_end_matches('\n').replace("koncepti", "konceptj");
        match verify_line_generic(&tampered, &[6, 7], 8) {
            PaWireVerdict::Corrupt(m) => assert!(m.contains("përputhet")),
            other => panic!("pritej Corrupt(CRC), erdhi {:?}", other),
        }
    }

    #[test]
    fn generic_legacy_counts_declared_and_alien_corrupt() {
        // 6 dhe 7 fusha pa vulë → legacy të deklaruara; 9 fusha → Corrupt.
        assert_eq!(verify_line_generic("a|b|c|d|e|f",     &[6, 7], 8), PaWireVerdict::LegacyOk);
        assert_eq!(verify_line_generic("a|b|c|d|e|f|g",   &[6, 7], 8), PaWireVerdict::LegacyOk);
        match verify_line_generic("a|b|c|d|e|f|g|h|i", &[6, 7], 8) {
            PaWireVerdict::Corrupt(m) => assert!(m.contains("numër fushash")),
            other => panic!("pritej Corrupt(fusha), erdhi {:?}", other),
        }
    }

    #[test]
    fn canonical_body_matches_encoder_body() {
        // Trupi kanonik = trupi që CRC-ja mbulon (asnjë divergjencë e fshehtë).
        let body = canonical_body("S", "00000000000000aa", 7, "xi:a", "yi:b");
        let line = encode_line("S", "00000000000000aa", 7, "xi:a|yi:b");
        assert!(line.starts_with(&format!("{}|c:", body)));
    }
}
