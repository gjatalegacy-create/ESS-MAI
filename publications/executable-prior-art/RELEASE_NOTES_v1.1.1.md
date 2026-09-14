# ESS-MAI Executable Prior Art v1.1.1

- Author and architect: **Bledar Gjata**
- Project: **ESS-MAI**
- Organization: **Gjata Legacy**
- License: **Apache-2.0**
- Release class: **POC-only defensive publication**

## Purpose of this patch release

Version 1.1.1 preserves the immutable v1.1.0 history and strengthens POC-005
and POC-006 under the explicit ESS-MAI Cargo POC experimental method:

```text
theory -> current source -> production mechanism -> isolated experiment
       -> baseline -> target -> ablation -> adversarial failure
       -> counterexample -> recovery boundary -> repeatability -> verdict
```

Each capsule now includes `CLAIM.md`, `SOURCE_RUNTIME_MAP.md`,
`EXPERIMENT_PROTOCOL.md`, explicit experimental success/failure/advancement,
and `FINAL_VERDICT.txt`. The complete private v189 source is not published.

## POC-005 result

### Experimental success

- three focal files remain byte-identical to v189;
- strict release/all-target build: PASS;
- strict detached tests: 15/15;
- independent runtime launches: 3/3 identical;
- one issued generation produces at most one post-authority result;
- copied replay, forged nonce, unknown generation and losing concurrent copies
  are refused;
- direct knowledge write remains held at `CompatibilityHold (-8)`.

### Experimental failure / counterexample

- the first strict `--all-targets` attempt exposed an incomplete authored
  adapter; the failure is preserved and the adapter alone was completed;
- two fresh generations for the same module and payload both reach `-8`;
- durable semantic idempotency, restart persistence and direct FFI-to-Vault
  commit are not materialised;
- no default-production in-repository caller for the legacy FFI exports was
  found in the declared v189 source search.

### Advancement method

Bind a durable semantic authorization key to the existing confirmed-verdict
transaction and consume it at an idempotent Shadow sink. The FFI child must
not inherit verdict or persistence authority.

## POC-006 result

### Experimental success

- four focal files remain byte-identical to v189;
- strict release/all-target build: PASS;
- detached tests: 30/30;
- independent runtime launches: 3/3 identical;
- valid required sealed negative history becomes `READY`, imports one entry,
  and hard-blocks a matching future candidate with score `0.000`;
- missing, corrupt and downgraded required history prevents PRO activation;
- v189 source preserves distinct constructive K+ and rigorous-negative K-
  writes under the superior GCL/Shadow verdict boundary.

### Experimental failure / counterexample

- the response exposes `negative_persisted` as a Boolean, not a complete typed
  negative-commit receipt;
- native WAL restart passes at component scope, while detached end-to-end
  Shadow terminate/reopen to next Quantum process remains unexecuted;
- K+ is source-evidenced and natively tested but is not a symmetric detached
  positive-history re-entry experiment in POC-006.

### Advancement method

Emit one typed negative-commit receipt only after the existing Shadow
transaction commits; bind asset token, transaction/cycle, GCL identity and
export digest; verify it in Quantum before import. Then test a real
terminate/reopen/next-cycle chain without moving persistence authority.

## Packaging reproducibility result

Both POCs passed clean strict builds, tests and three-run behavioral checks in
multiple build roots. Identical code/Cargo inputs produced identical semantic
receipts, but the Windows executable SHA-256 values differed across clean
build roots. This release therefore claims behavioral reproducibility and
records every runtime-witness hash; it does **not** claim bit-for-bit binary
reproducibility. Path-remapping and reproducible-linker controls are reserved
for a separate packaging experiment and do not alter either theory verdict.

## Theory and prior-art boundary

Existing capability theory, reference-monitor principles, failure-driven
reminding, negative-memory systems and positive/negative knowledge sharing are
declared as prior art. The ESS-MAI contribution and bounded innovation are
stated separately from those foundations.

The Business Magazine record dated 2026-06-03 is used only as public
conceptual attribution of “Dija Negative” to Bledar Gjata and ESS-MAI. It is
not treated as proof that the later implementation already existed. Later
independent work is declared without alleging copying or making a universal
priority claim.

## Claim boundary

Both capsules are classified:

```text
NARROW_CLAIM_STATUS=POC_READY
GLOBAL_THEORY_STATUS=THEORY_NOT_MATERIALISED_GLOBALLY
POC_STATUS=PARTIAL_POC
```

This seals only the disclosed POC questions. It does not declare the complete
ESS-MAI v189 architecture closed, production-certified or fully materialised.
