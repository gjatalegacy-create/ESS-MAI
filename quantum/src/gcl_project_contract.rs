//! v1.6.4 — GCL Scientific Project Continuum.
//!
//! Identiteti i projektit lind në Light/APUPK, konteksti konfirmohet nga
//! Shadow main, Quantum procedon TRL/SRK/PIM/NPIM nën të njëjtin GCL process,
//! dhe Shadow e lidh evidencën me verdiktin suprem. Ky modul nuk krijon
//! autoritet të ri; ai vetëm kanonizon materialin që hyn në GCL/Living Trust.

use sha2::{Digest, Sha256};

pub const GCL_PROJECT_CONTRACT_VERSION: u32 = 0x0001_0603;
pub const SOVEREIGN_SEAL_XOR: u32 = 0xA5A5;
pub const SOVEREIGN_SEAL_MASK: u32 = 0x0000_FFFF;
pub const SOVEREIGN_SEAL_VALUE: u32 = 500;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectContextMaterial<'a> {
    pub project_id: u64,
    pub user_id: u64,
    pub trace_id: u64,
    pub revision: u32,
    pub project_title: &'a str,
    pub content_sha256: &'a str,
    pub light_sovereign_flags: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProjectVerdictMaterial {
    pub project_id: u64,
    pub project_status: u8,
    /// SHA-256 i plotë i kontekstit APUPK; u64 përdoret vetëm si indeks legacy.
    pub project_context_sha256: [u8; 32],
    /// SHA-256 i plotë i evidencës shkencore; ky është identiteti që hyn në Trust.
    pub project_evidence_sha256: [u8; 32],
    pub novel_factualized: bool,
    pub novel_trl_level: u8,
    pub novel_proof_score: u32,
    pub novel_rejection: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProjectEvidenceMaterial<'a> {
    pub context_sha256: &'a str,
    /// Materiali i pyetjes shkencore që Quantum procedoi në Digital Lab.
    pub project_title: &'a str,
    pub domain: &'a str,
    pub hypothesis: &'a str,
    pub assumptions: &'a [String],
    pub gcl_process_digest: u64,
    pub trl_level: u8,
    pub trl_passed: bool,
    pub trl_confidence: u32,
    pub trl_reproducibility: u32,
    pub lab_test_id: &'a str,
    pub findings: &'a str,
    pub description: &'a str,
    /// Çdo element është `kind || len || bytes`; rendi është pjesë e provës.
    pub files: &'a [(u8, Vec<u8>)],
}


/// Formula ekzistuese e APUPK për trace-in e projektit. Duke e mbajtur në
/// kontratën byte-identike, Quantum dhe Shadow provojnë që titulli shkencor i
/// paketuar është i njëjti titull që Light përdori kur lindi identitetin.
pub fn project_trace_id(project_id: u64, user_id: u64, project_title: &str) -> u64 {
    project_title.as_bytes().iter().fold(
        project_id.wrapping_add(user_id),
        |hash, byte| hash.wrapping_mul(31).wrapping_add(u64::from(*byte)),
    )
}

pub fn seal_is_500(flags: u32) -> bool {
    flags != 0 && ((flags & SOVEREIGN_SEAL_MASK) ^ SOVEREIGN_SEAL_XOR) == SOVEREIGN_SEAL_VALUE
}

pub fn is_hex_32(value: &str) -> bool {
    value.len() == 64 && value.chars().all(|c| c.is_ascii_hexdigit())
}

pub fn context_sha256(material: &ProjectContextMaterial<'_>) -> String {
    let mut h = Sha256::new();
    h.update(b"ESS_MAI_GCL_PROJECT_CONTEXT_V164");
    h.update(GCL_PROJECT_CONTRACT_VERSION.to_le_bytes());
    h.update(material.project_id.to_le_bytes());
    h.update(material.user_id.to_le_bytes());
    h.update(material.trace_id.to_le_bytes());
    h.update(material.revision.to_le_bytes());
    update_text(&mut h, material.project_title);
    h.update((material.content_sha256.len() as u64).to_le_bytes());
    h.update(material.content_sha256.as_bytes());
    h.update(material.light_sovereign_flags.to_le_bytes());
    hex(h.finalize().as_slice())
}

pub fn evidence_sha256(material: &ProjectEvidenceMaterial<'_>) -> String {
    let mut h = Sha256::new();
    h.update(b"ESS_MAI_GCL_SCIENTIFIC_PROJECT_EVIDENCE_V164");
    h.update(GCL_PROJECT_CONTRACT_VERSION.to_le_bytes());
    h.update((material.context_sha256.len() as u64).to_le_bytes());
    h.update(material.context_sha256.as_bytes());
    update_text(&mut h, material.project_title);
    update_text(&mut h, material.domain);
    update_text(&mut h, material.hypothesis);
    h.update((material.assumptions.len() as u64).to_le_bytes());
    for assumption in material.assumptions {
        update_text(&mut h, assumption);
    }
    h.update(material.gcl_process_digest.to_le_bytes());
    h.update([material.trl_level]);
    h.update([material.trl_passed as u8]);
    h.update(material.trl_confidence.to_le_bytes());
    h.update(material.trl_reproducibility.to_le_bytes());
    update_text(&mut h, material.lab_test_id);
    update_text(&mut h, material.findings);
    update_text(&mut h, material.description);
    h.update((material.files.len() as u64).to_le_bytes());
    for (kind, bytes) in material.files {
        h.update([*kind]);
        h.update((bytes.len() as u64).to_le_bytes());
        h.update(bytes);
    }
    hex(h.finalize().as_slice())
}


/// SHA-256 i plotë i rezultatit shkencor që futet në Living Trust.
/// Statusi Novel/Hold/Negative nuk mund të ndryshohet pas pulsit SHA-256 dhe
/// identiteti i projektit nuk shkurtohet në 64 bit.
pub fn verdict_sha256_or_zero(material: ProjectVerdictMaterial) -> [u8; 32] {
    let absent = material.project_id == 0
        && material.project_status == 0
        && material.project_context_sha256 == [0u8; 32]
        && material.project_evidence_sha256 == [0u8; 32]
        && !material.novel_factualized
        && material.novel_trl_level == 0
        && material.novel_proof_score == 0
        && material.novel_rejection == 0;
    match absent {
        true => [0u8; 32],
        false => verdict_sha256(material),
    }
}

pub fn verdict_sha256(material: ProjectVerdictMaterial) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(b"ESS_MAI_GCL_SCIENTIFIC_PROJECT_VERDICT_V164");
    h.update(GCL_PROJECT_CONTRACT_VERSION.to_le_bytes());
    h.update(material.project_id.to_le_bytes());
    h.update([material.project_status]);
    h.update(material.project_context_sha256);
    h.update(material.project_evidence_sha256);
    h.update([material.novel_factualized as u8]);
    h.update([material.novel_trl_level]);
    h.update(material.novel_proof_score.to_le_bytes());
    h.update([material.novel_rejection]);
    let digest = h.finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(&digest);
    out
}

/// Indeks legacy 64-bit për raporte/compatibility. Nuk përdoret si identitet
/// kriptografik në Living Trust.
pub fn verdict_digest_or_zero(material: ProjectVerdictMaterial) -> u64 {
    let full = verdict_sha256_or_zero(material);
    digest64_bytes(&full)
}

pub fn parse_sha256(value: &str) -> Option<[u8; 32]> {
    match is_hex_32(value) {
        true => {}
        false => return None,
    }
    let bytes = value.as_bytes();
    let mut out = [0u8; 32];
    for i in 0..32 {
        let hi = nibble(bytes[i * 2])?;
        let lo = nibble(bytes[i * 2 + 1])?;
        out[i] = (hi << 4) | lo;
    }
    Some(out)
}

pub fn sha256_hex(value: &[u8; 32]) -> String {
    hex(value)
}

pub fn digest64_bytes(value: &[u8; 32]) -> u64 {
    let mut first = [0u8; 8];
    first.copy_from_slice(&value[..8]);
    u64::from_le_bytes(first)
}

/// Kodim kanonik i skedarëve të evidencës për transportin e kufizuar
/// Light → Quantum. Formati binar për çdo hyrje është kind:u8 | len:u32 LE |
/// bytes; i gjithë blob-i paraqitet hex që të mos thyejë delimiterët e bus-it.
pub fn encode_files_hex(files: &[(u8, Vec<u8>)]) -> String {
    let mut raw = Vec::new();
    raw.extend_from_slice(&(files.len() as u32).to_le_bytes());
    for (kind, bytes) in files {
        raw.push(*kind);
        raw.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
        raw.extend_from_slice(bytes);
    }
    hex(&raw)
}

pub fn decode_files_hex(value: &str) -> Option<Vec<(u8, Vec<u8>)>> {
    let raw = decode_hex(value)?;
    let mut offset = 0usize;
    let count = read_u32(&raw, &mut offset)? as usize;
    match count <= 4_096 {
        true => {}
        false => return None,
    }
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        let kind = *raw.get(offset)?;
        offset = offset.checked_add(1)?;
        let len = read_u32(&raw, &mut offset)? as usize;
        let end = offset.checked_add(len)?;
        let bytes = raw.get(offset..end)?.to_vec();
        offset = end;
        out.push((kind, bytes));
    }
    match offset == raw.len() {
        true => Some(out),
        false => None,
    }
}

pub fn digest64(hex_sha256: &str) -> u64 {
    match decode_first_8(hex_sha256) {
        Some(bytes) => u64::from_le_bytes(bytes),
        None => 0,
    }
}

fn update_text(h: &mut Sha256, value: &str) {
    h.update((value.len() as u64).to_le_bytes());
    h.update(value.as_bytes());
}

fn hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0F) as usize] as char);
    }
    out
}


fn decode_hex(value: &str) -> Option<Vec<u8>> {
    match value.len() % 2 == 0 && value.as_bytes().iter().all(|b| b.is_ascii_hexdigit()) {
        true => {}
        false => return None,
    }
    let bytes = value.as_bytes();
    let mut out = Vec::with_capacity(bytes.len() / 2);
    for pair in bytes.chunks_exact(2) {
        out.push((nibble(pair[0])? << 4) | nibble(pair[1])?);
    }
    Some(out)
}

fn read_u32(bytes: &[u8], offset: &mut usize) -> Option<u32> {
    let end = offset.checked_add(4)?;
    let raw: [u8; 4] = bytes.get(*offset..end)?.try_into().ok()?;
    *offset = end;
    Some(u32::from_le_bytes(raw))
}

fn decode_first_8(value: &str) -> Option<[u8; 8]> {
    match is_hex_32(value) {
        true => {}
        false => return None,
    }
    let bytes = value.as_bytes();
    let mut out = [0u8; 8];
    for i in 0..8 {
        let hi = nibble(bytes[i * 2])?;
        let lo = nibble(bytes[i * 2 + 1])?;
        out[i] = (hi << 4) | lo;
    }
    Some(out)
}

fn nibble(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => Some(value - b'0'),
        b'a'..=b'f' => Some(value - b'a' + 10),
        b'A'..=b'F' => Some(value - b'A' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_and_evidence_are_deterministic() {
        let c = ProjectContextMaterial {
            project_id: 7, user_id: 9, trace_id: 11, revision: 0,
            project_title: "motor novel",
            content_sha256: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            light_sovereign_flags: 0xA451,
        };
        let context = context_sha256(&c);
        assert!(is_hex_32(&context));
        assert_eq!(context, context_sha256(&c));
        let files = vec![(3u8, b"document".to_vec())];
        let assumptions = vec!["A1".to_string(), "A2".to_string()];
        let e = ProjectEvidenceMaterial {
            context_sha256: &context, project_title: "motor novel", domain: "physics",
            hypothesis: "H1", assumptions: &assumptions,
            gcl_process_digest: 12, trl_level: 3,
            trl_passed: true, trl_confidence: 8_000, trl_reproducibility: 9_000,
            lab_test_id: "LAB", findings: "verified", description: "how it emerged",
            files: &files,
        };
        let evidence = evidence_sha256(&e);
        assert!(is_hex_32(&evidence));
        assert_ne!(digest64(&evidence), 0);
        let verdict = verdict_digest_or_zero(ProjectVerdictMaterial {
            project_id: 7,
            project_status: 4,
            project_context_sha256: parse_sha256(&context).unwrap(),
            project_evidence_sha256: parse_sha256(&evidence).unwrap(),
            novel_factualized: true,
            novel_trl_level: 4,
            novel_proof_score: 8_000,
            novel_rejection: 0,
        });
        assert_ne!(verdict, 0);
        assert_eq!(verdict_digest_or_zero(ProjectVerdictMaterial {
            project_id: 0,
            project_status: 0,
            project_context_sha256: [0u8; 32],
            project_evidence_sha256: [0u8; 32],
            novel_factualized: false,
            novel_trl_level: 0,
            novel_proof_score: 0,
            novel_rejection: 0,
        }), 0);
    }


    #[test]
    fn evidence_file_transport_roundtrips() {
        let files = vec![(1u8, vec![0x89, 0x50, 0x4e, 0x47]), (3u8, b"pdf".to_vec())];
        let encoded = encode_files_hex(&files);
        assert_eq!(decode_files_hex(&encoded), Some(files));
        assert!(decode_files_hex("0").is_none());
    }

    #[test]
    fn seal_500_is_structural() {
        assert_eq!(project_trace_id(7, 9, "motor"),
            "motor".as_bytes().iter().fold(16u64, |h, b| h.wrapping_mul(31).wrapping_add(u64::from(*b))));
        assert!(seal_is_500(0xA451));
        assert!(!seal_is_500(0));
    }
}
