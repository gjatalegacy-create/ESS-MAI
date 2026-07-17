# ESS-MAI v1.5.6

- Deklaroi formulën e PD: `i + U → i₀`, `PD(i₀) → output + iZ`, `output + iZ → next i₀`.
- Shtoi kontratën identike `pd_continuum_contract.rs` në tri platformat.
- Ndarë `PdVerifiedOutput`, `PdIzCompletion` dhe `PdNextI0`.
- Lidhi output-in real të PD me digest-in; manipulimi i pyetjes/output-it ndryshon iZ dhe next i₀.
- Deklaroi `GclRole::PdContinuum` si rol të detyrueshëm në Reasoning.
- Deklaroi në Light kontinuumin `MiniAlgorithm → Algorithm → PD` dhe e përdor në runtime health.
- Ndaloi çdo pre-seal para mbylljes së Spine 9 / Layer 1/2/3.
- Hoqi krijimin e fshehtë të receipt-it bosh gjatë hapjes së sesionit.
- Ndarë `session_id` nga `parent_i₀`; i₀ merret nga Primitive Anchor real i Light.
- Zgjeroi Shadow receipt me `pd_continuum_activation_digest` dhe lidhjen e kandidatit.
- Zgjeroi handoff-in PD Quantum→PD Light me output, iZ dhe completion digest.
- Mbajti masat e rrjedhës PD/MPRO si `u32` fixed-point, shkalla 10_000.
- Zgjeroi komandën emocionale me fazën `PD_IZ_COMPLETED` / `CONTINUUM_PULSE`.
- Përditësoi versionet Cargo/Tauri në 1.5.6.
- Përditësoi CI gate për `read_primitive_context` (`i₀ + Xi/Yi`).
