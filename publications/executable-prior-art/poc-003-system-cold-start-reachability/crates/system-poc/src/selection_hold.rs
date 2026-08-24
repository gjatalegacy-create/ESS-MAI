//! Neutral, cross-process hash holds for Besa/Asht read-only selections.
//!
//! A hold is not knowledge, a verdict, or a vault write. It preserves the
//! exact bounded response until Nbes verifies that the same evidence returned
//! through the authoritative VDS cycle.

use shadow_contracts::{
    decode_selection_response, encode_selection_response, is_canonical_sha256, sha256_hex,
    KnowledgeSelectionResponseWire, MuscleEvidenceWire, SelectionRequester,
};
use std::fs::{self, OpenOptions};
use std::io::{Read as _, Write as _};
use std::path::{Path, PathBuf};

const HOLD_DIRECTORY: &str = "selection_holds";
const HOLD_LOCK: &str = ".selection_hold.lock";

struct SelectionProcessLock {
    path: PathBuf,
}

impl SelectionProcessLock {
    fn acquire(root: &Path) -> Result<Self, String> {
        fs::create_dir_all(root)
            .map_err(|error| format!("selection hold directory failed: {error}"))?;
        let path = root.join(HOLD_LOCK);
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .and_then(|mut file| {
                file.write_all(std::process::id().to_string().as_bytes())?;
                file.sync_all()
            })
            .map_err(|error| format!("selection hold lock refused: {error}"))?;
        Ok(Self { path })
    }
}

impl Drop for SelectionProcessLock {
    fn drop(&mut self) {
        match fs::remove_file(&self.path) {
            Ok(()) => {}
            Err(error) => eprintln!(
                "[SELECTION_HOLD] cleanup defect: lock {} remained: {error}",
                self.path.display()
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SelectionHoldReceipt {
    pub hold_id: String,
    pub hold_sha256: String,
    pub record_path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct SelectionHoldStore {
    root: PathBuf,
}

impl SelectionHoldStore {
    pub fn under_handoff(handoff_root: impl AsRef<Path>) -> Self {
        Self {
            root: handoff_root.as_ref().join(HOLD_DIRECTORY),
        }
    }

    fn record_path(&self, hold_id: &str) -> Result<PathBuf, String> {
        let safe = !hold_id.is_empty()
            && hold_id
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_');
        match safe {
            true => Ok(self.root.join(format!("{hold_id}.hold"))),
            false => Err("selection hold id is not path-safe".to_string()),
        }
    }

    pub fn recompute_hold_sha256(response: &KnowledgeSelectionResponseWire) -> String {
        let requester = [response.requester as u8];
        let anchor = response.primitive_anchor.to_le_bytes();
        let laws = [
            response.law_seal.to_le_bytes(),
            response.system_laws_seal.to_le_bytes(),
            response.expires_at_ns.to_le_bytes(),
        ];
        sha256_hex(&[
            b"ESSMAI/SELECTION/HOLD/V178",
            &requester,
            response.session_id.as_bytes(),
            response.parent_i0.as_bytes(),
            &anchor,
            response.split_sha256.as_bytes(),
            response.upstream_hold_sha256.as_deref().unwrap_or("").as_bytes(),
            response.request_sha256.as_bytes(),
            response.selection_sha256.as_bytes(),
            response.hold_id.as_bytes(),
            &laws[0],
            &laws[1],
            &laws[2],
        ])
    }

    pub fn stage(
        &self,
        response: &KnowledgeSelectionResponseWire,
    ) -> Result<SelectionHoldReceipt, String> {
        match (
            is_canonical_sha256(&response.hold_sha256),
            response.hold_sha256 == Self::recompute_hold_sha256(response),
        ) {
            (true, true) => {}
            (false, _) => return Err("selection hold SHA-256 is not canonical".to_string()),
            (_, false) => return Err("selection hold SHA-256 does not recompute".to_string()),
        }
        let _lock = SelectionProcessLock::acquire(&self.root)?;
        let record_path = self.record_path(&response.hold_id)?;
        match record_path.exists() {
            true => return Err("selection hold replay refused".to_string()),
            false => {}
        }
        let temporary = self.root.join(format!(
            ".{}.{}.tmp",
            response.hold_id,
            std::process::id()
        ));
        let bytes = encode_selection_response(response);
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)
            .map_err(|error| format!("selection hold temp open failed: {error}"))?;
        file.write_all(&bytes)
            .map_err(|error| format!("selection hold temp write failed: {error}"))?;
        file.sync_all()
            .map_err(|error| format!("selection hold temp sync failed: {error}"))?;
        drop(file);
        fs::rename(&temporary, &record_path)
            .map_err(|error| format!("selection hold atomic publish failed: {error}"))?;
        Ok(SelectionHoldReceipt {
            hold_id: response.hold_id.clone(),
            hold_sha256: response.hold_sha256.clone(),
            record_path,
        })
    }

    pub fn load_bound(
        &self,
        hold_id: &str,
        session_id: &str,
        primitive_anchor: u64,
        split_sha256: &str,
        expected_hold_sha256: &str,
    ) -> Result<KnowledgeSelectionResponseWire, String> {
        let path = self.record_path(hold_id)?;
        let mut file = OpenOptions::new()
            .read(true)
            .open(&path)
            .map_err(|error| format!("selection hold open failed: {error}"))?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)
            .map_err(|error| format!("selection hold read failed: {error}"))?;
        let response = decode_selection_response(&bytes)
            .map_err(|error| format!("selection hold decode failed: {error}"))?;
        match (
            response.hold_id == hold_id,
            response.session_id == session_id,
            response.primitive_anchor == primitive_anchor,
            response.split_sha256 == split_sha256,
            response.hold_sha256 == expected_hold_sha256,
            response.hold_sha256 == Self::recompute_hold_sha256(&response),
        ) {
            (true, true, true, true, true, true) => Ok(response),
            (false, _, _, _, _, _) => Err("selection hold id mismatch".to_string()),
            (_, false, _, _, _, _) => Err("selection hold session mismatch".to_string()),
            (_, _, false, _, _, _) => Err("selection hold primitive anchor mismatch".to_string()),
            (_, _, _, false, _, _) => Err("selection hold split mismatch".to_string()),
            (_, _, _, _, false, _) => Err("selection hold evidence SHA mismatch".to_string()),
            (_, _, _, _, _, false) => Err("selection hold record SHA mismatch".to_string()),
        }
    }

    pub fn verify_muscle_evidence(
        &self,
        session_id: &str,
        primitive_anchor: u64,
        evidence: &MuscleEvidenceWire,
        now_ns: u64,
    ) -> Result<(KnowledgeSelectionResponseWire, KnowledgeSelectionResponseWire), String> {
        match evidence.verifies_internal() {
            true => {}
            false => return Err("muscle evidence digest invalid".to_string()),
        }
        let besa = self.load_bound(
            &evidence.besa_hold_id,
            session_id,
            primitive_anchor,
            &evidence.split_sha256,
            &evidence.besa_hold_sha256,
        )?;
        let asht = self.load_bound(
            &evidence.asht_hold_id,
            session_id,
            primitive_anchor,
            &evidence.split_sha256,
            &evidence.asht_hold_sha256,
        )?;
        match (
            besa.requester,
            asht.requester,
            besa.selection_sha256 == evidence.besa_selection_sha256,
            asht.selection_sha256 == evidence.asht_selection_sha256,
            asht.upstream_hold_sha256.as_deref() == Some(besa.hold_sha256.as_str()),
            besa.expires_at_ns >= now_ns,
            asht.expires_at_ns >= now_ns,
        ) {
            (
                SelectionRequester::BesaNlight,
                SelectionRequester::AshtQuantum,
                true,
                true,
                true,
                true,
                true,
            ) => Ok((besa, asht)),
            (SelectionRequester::AshtQuantum, _, _, _, _, _, _)
            | (_, SelectionRequester::BesaNlight, _, _, _, _, _) => {
                Err("selection hold requester hierarchy mismatch".to_string())
            }
            (_, _, false, _, _, _, _) => Err("Besa selection digest mismatch".to_string()),
            (_, _, _, false, _, _, _) => Err("Asht selection digest mismatch".to_string()),
            (_, _, _, _, false, _, _) => Err("Asht upstream Besa hold mismatch".to_string()),
            (_, _, _, _, _, false, _) => Err("Besa selection hold expired".to_string()),
            (_, _, _, _, _, _, false) => Err("Asht selection hold expired".to_string()),
        }
    }

    pub fn cleanup_after_commit(&self, hold_ids: &[&str]) -> Result<(), String> {
        let _lock = SelectionProcessLock::acquire(&self.root)?;
        for hold_id in hold_ids {
            let path = self.record_path(hold_id)?;
            fs::remove_file(&path).map_err(|error| {
                format!("selection hold cleanup failed for {}: {error}", path.display())
            })?;
        }
        Ok(())
    }

    pub fn purge_expired(&self, now_ns: u64) -> Result<usize, String> {
        let _lock = SelectionProcessLock::acquire(&self.root)?;
        let mut purged = 0usize;
        for entry in fs::read_dir(&self.root)
            .map_err(|error| format!("selection hold scan failed: {error}"))?
        {
            let entry = entry.map_err(|error| format!("selection hold entry failed: {error}"))?;
            let extension_is_hold = entry.path().extension().and_then(|value| value.to_str())
                == Some("hold");
            match extension_is_hold {
                false => {}
                true => {
                    let bytes = fs::read(entry.path())
                        .map_err(|error| format!("selection hold purge read failed: {error}"))?;
                    let response = decode_selection_response(&bytes)
                        .map_err(|error| format!("selection hold purge decode failed: {error}"))?;
                    match response.expires_at_ns < now_ns {
                        true => {
                            fs::remove_file(entry.path()).map_err(|error| {
                                format!("selection hold purge delete failed: {error}")
                            })?;
                            purged = purged.saturating_add(1);
                        }
                        false => {}
                    }
                }
            }
        }
        Ok(purged)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn response(root: &Path) -> (SelectionHoldStore, KnowledgeSelectionResponseWire) {
        let store = SelectionHoldStore::under_handoff(root);
        let mut response = KnowledgeSelectionResponseWire {
            version: shadow_contracts::PROTOCOL_VERSION,
            requester: SelectionRequester::BesaNlight,
            session_id: "session".into(),
            parent_i0: "i0".into(),
            primitive_anchor: 7,
            split_sha256: "11".repeat(32),
            upstream_hold_sha256: None,
            request_sha256: "22".repeat(32),
            selection_sha256: "33".repeat(32),
            hold_id: "hold-besa".into(),
            hold_sha256: String::new(),
            positive: Vec::new(),
            negative: Vec::new(),
            law_seal: 1,
            system_laws_seal: 2,
            expires_at_ns: u64::MAX,
        };
        response.hold_sha256 = SelectionHoldStore::recompute_hold_sha256(&response);
        (store, response)
    }

    #[test]
    fn staged_hold_roundtrips_and_replay_is_refused() {
        let root = std::env::temp_dir().join(format!(
            "essmai-selection-hold-{}",
            shadow_contracts::fnv1a64(format!("{:?}", std::time::SystemTime::now()).as_bytes())
        ));
        let (store, response) = response(&root);
        let receipt = store.stage(&response).expect("stage hold");
        assert_eq!(receipt.hold_id, response.hold_id);
        assert!(store.stage(&response).is_err());
        let loaded = store
            .load_bound(
                &response.hold_id,
                &response.session_id,
                response.primitive_anchor,
                &response.split_sha256,
                &response.hold_sha256,
            )
            .expect("load hold");
        assert_eq!(loaded, response);
        std::fs::remove_dir_all(root).expect("remove test hold root");
    }

    #[test]
    fn nbes_verifies_besa_asht_hierarchy_and_cleans_only_after_commit_signal() {
        let root = std::env::temp_dir().join(format!(
            "essmai-nbes-holds-{}",
            shadow_contracts::fnv1a64(format!("{:?}", std::time::SystemTime::now()).as_bytes())
        ));
        let (store, besa) = response(&root);
        store.stage(&besa).expect("stage Besa hold");
        let mut asht = besa.clone();
        asht.requester = SelectionRequester::AshtQuantum;
        asht.hold_id = "hold-asht".into();
        asht.selection_sha256 = "44".repeat(32);
        asht.upstream_hold_sha256 = Some(besa.hold_sha256.clone());
        asht.hold_sha256 = SelectionHoldStore::recompute_hold_sha256(&asht);
        store.stage(&asht).expect("stage Asht hold");

        let mut evidence = MuscleEvidenceWire {
            besa_hold_id: besa.hold_id.clone(),
            besa_hold_sha256: besa.hold_sha256.clone(),
            besa_selection_sha256: besa.selection_sha256.clone(),
            besa_attestation_sha256: "55".repeat(32),
            asht_hold_id: asht.hold_id.clone(),
            asht_hold_sha256: asht.hold_sha256.clone(),
            asht_selection_sha256: asht.selection_sha256.clone(),
            asht_attestation_sha256: "66".repeat(32),
            split_sha256: besa.split_sha256.clone(),
            combined_sha256: String::new(),
        };
        evidence.combined_sha256 = evidence.recompute_combined_sha256();
        store
            .verify_muscle_evidence(
                &besa.session_id,
                besa.primitive_anchor,
                &evidence,
                1,
            )
            .expect("Nbes verifies both bound holds");
        store
            .cleanup_after_commit(&[&besa.hold_id, &asht.hold_id])
            .expect("post-commit cleanup");
        assert!(store.load_bound(
            &besa.hold_id,
            &besa.session_id,
            besa.primitive_anchor,
            &besa.split_sha256,
            &besa.hold_sha256,
        ).is_err());
        std::fs::remove_dir_all(root).expect("remove Nbes test root");
    }
}
