// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  SHADOW_DESTFAKE.RS — ELIMINIMI I INFOS PA GJURMË                     ║
// ║  Gjata Legacy™ | Bledar Gjata                                        ║
// ║                                                                          ║
// ║  Inputet pa gjurmueshmëri algoritmike NUK janë dije → trajtohen këtu.  ║
// ║  Politikë DETERMINISTE (jo probabilitet): arsyeja → veprim përmes      ║
// ║  një tabele të prerë. Eliminim matematik, jo gjykim heuristik.        ║
// ║                                                                          ║
// ║    Purge   — hidhet menjëherë (default); numërohet te ledger.    ║
// ║    Isolate — vendoset në karantinë (numërohet; s'persiston si dije).  ║
// ║    Mark    — shënohet si i pavërtetuar (numërohet).                   ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::knowledge_lineage::ledger;
use crate::luvik::LuvikReject;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::OnceLock;

/// Veprimi i eliminimit — i koduar si u8 për përcaktueshmëri.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum DestFakeAction {
    Purge   = 0,
    Isolate = 1,
    Mark    = 2,
}

#[derive(Debug, Default)]
pub struct DestFake {
    purged:   AtomicU64,
    isolated: AtomicU64,
    marked:   AtomicU64,
}

impl DestFake {
    const fn new() -> Self {
        Self {
            purged:   AtomicU64::new(0),
            isolated: AtomicU64::new(0),
            marked:   AtomicU64::new(0),
        }
    }

    /// TABELË E PRERË arsye→veprim (ZERO if/else mbi vendimin).
    ///: çdo refuzim → Purge (eliminim i drejtpërdrejtë).
    pub fn action_for(_reject: LuvikReject) -> DestFakeAction {
        // E njëjta politikë për të dyja arsyet aktuale; tabela lë vend zgjerimi.
        const TABLE: [DestFakeAction; 2] = [DestFakeAction::Purge, DestFakeAction::Purge];
        TABLE[(_reject as usize).min(1)]
    }

    /// Pika kryesore: trajton një refuzim Luvik → zgjedh veprimin → ekzekuton.
    pub fn on_reject(&self, reject: LuvikReject) -> DestFakeAction {
        let action = Self::action_for(reject);
        self.execute(action);
        action
    }

    /// Ekzekuton veprimin. (Match = dispatch veprimi, jo gjykim vendimi.)
    pub fn execute(&self, action: DestFakeAction) {
        match action {
            DestFakeAction::Purge => {
                self.purged.fetch_add(1, Ordering::Relaxed);
                ledger().note_purge(); // info hidhet — s'persiston si dije
            }
            DestFakeAction::Isolate => {
                self.isolated.fetch_add(1, Ordering::Relaxed);
                ledger().note_purge();
            }
            DestFakeAction::Mark => {
                self.marked.fetch_add(1, Ordering::Relaxed);
            }
        }
    }

    pub fn purged_count(&self)   -> u64 { self.purged.load(Ordering::Relaxed) }
    pub fn isolated_count(&self) -> u64 { self.isolated.load(Ordering::Relaxed) }
    pub fn marked_count(&self)   -> u64 { self.marked.load(Ordering::Relaxed) }
}

// ── Singleton global (një autoritet eliminimi për procesin) ─────────────────
static DESTFAKE: OnceLock<DestFake> = OnceLock::new();

pub fn destfake() -> &'static DestFake {
    DESTFAKE.get_or_init(DestFake::new)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reject_maps_to_purge() {
        assert_eq!(DestFake::action_for(LuvikReject::NoAlgorithmicTrace), DestFakeAction::Purge);
        assert_eq!(DestFake::action_for(LuvikReject::SealStripped), DestFakeAction::Purge);
    }

    #[test]
    fn on_reject_purges_and_counts() {
        let df = DestFake::new();
        let before = df.purged_count();
        let action = df.on_reject(LuvikReject::NoAlgorithmicTrace);
        assert_eq!(action, DestFakeAction::Purge);
        assert_eq!(df.purged_count(), before + 1);
    }

    #[test]
    fn isolate_and_mark_count_separately() {
        let df = DestFake::new();
        df.execute(DestFakeAction::Isolate);
        df.execute(DestFakeAction::Mark);
        df.execute(DestFakeAction::Mark);
        assert_eq!(df.isolated_count(), 1);
        assert_eq!(df.marked_count(), 2);
    }
}
