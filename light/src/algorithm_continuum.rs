// ╔══════════════════════════════════════════════════════════════════════════╗
// ║ ALGORITHM_CONTINUUM — deklarimi Mini → Algorithm → PD/iZ               ║
// ║ ESS-MAI v1.5.6                                                        ║
// ╚══════════════════════════════════════════════════════════════════════════╝

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlgorithmTier {
    MiniAlgorithm,
    Algorithm,
    PdContinuum,
}

impl AlgorithmTier {
    pub fn label(self) -> &'static str {
        match self {
            Self::MiniAlgorithm => "MINI_ALGORITHM",
            Self::Algorithm => "ALGORITHM",
            Self::PdContinuum => "PD_I0_TO_IZ",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlgorithmContinuumDeclaration {
    pub mini_declared: bool,
    pub algorithm_declared: bool,
    pub pd_continuum_declared: bool,
    pub question_increment_units: u32,
    pub contract_version: u32,
}

impl AlgorithmContinuumDeclaration {
    pub const fn canonical() -> Self {
        Self {
            mini_declared: true,
            algorithm_declared: true,
            pd_continuum_declared: true,
            question_increment_units:
                crate::pd_continuum_contract::QUESTION_INCREMENT_UNITS,
            contract_version:
                crate::pd_continuum_contract::PD_CONTINUUM_CONTRACT_VERSION,
        }
    }

    pub fn is_complete(self) -> bool {
        self.mini_declared
            && self.algorithm_declared
            && self.pd_continuum_declared
            && self.question_increment_units == 1
            && self.contract_version
                == crate::pd_continuum_contract::PD_CONTINUUM_CONTRACT_VERSION
    }

    pub fn route(self) -> [AlgorithmTier; 3] {
        [
            AlgorithmTier::MiniAlgorithm,
            AlgorithmTier::Algorithm,
            AlgorithmTier::PdContinuum,
        ]
    }
}

pub fn is_complete() -> bool {
    AlgorithmContinuumDeclaration::canonical().is_complete()
}

/// Gjurmë runtime e deklarimit: mini-algoritmi nuk mbetet warning i shkëputur,
/// por shfaqet si pjesë e të njëjtit kontinuum që mbyllet te PD.
pub fn route_signature() -> String {
    AlgorithmContinuumDeclaration::canonical()
        .route()
        .iter()
        .map(|tier| tier.label())
        .collect::<Vec<_>>()
        .join("→")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mini_algorithm_pd_is_one_declared_continuum() {
        let d = AlgorithmContinuumDeclaration::canonical();
        assert!(d.is_complete());
        assert_eq!(d.route()[2].label(), "PD_I0_TO_IZ");
    }
}
