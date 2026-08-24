# Priority and Timestamp Timeline

This file records evidence status; it does not create legal priority by assertion.

## 2026-06-03 — interview claim

Status: `PUBLIC_RECORD_VERIFIED_CLAIM_MAPPING_PENDING`

Business Magazine published “A jemi drejt një AI Sovrane? Bledar Gjata dhe vizioni ambicioz pas ESS-MAI” on 2026-06-03:

`https://businessmag.al/a-jemi-drejt-nje-ai-sovrane-bledar-gjata-dhe-vizioni-ambicioz-pas-ess-mai/`

The page publicly identifies Bledar Gjata and ESS-MAI and describes the project as an early-stage system. The exact LAW-0 claims and the exact files in this capsule have not yet been mapped to article passages.

Therefore this capsule records the article as verified project-level public context, but does not label it “first-to-disclose” for every LAW-0 claim.

Evidence still required for a narrower LAW-0 priority claim:

- archived snapshot or other independently retrievable record;
- exact excerpt/claim mapping, within copyright limits;
- explanation of how the disclosed claim corresponds to this bounded LAW-0 artifact.

## 2026-08-23 — local POC verification

Status: `LOCALLY_VERIFIED_NOT_INDEPENDENTLY_TIMESTAMPED`

On this date:

- the five-file production crate closure was extracted and verified byte-identical;
- a clean locked offline Cargo build passed;
- 19 unit tests passed;
- five deterministic experiment runs passed and reproduced both success and failure;
- the v189 tree digest remained unchanged;
- local artifact hashes were generated.

Local filesystem time and a self-generated SHA-256 manifest prove internal integrity relative to this snapshot. By themselves, they do not provide an independent public timestamp.

## 2026-08-24 — superseding public release candidate

Status: `LOCALLY_VALIDATED_AWAITING_REPOSITORY_TAG`

The Apache-2.0 capsule version 0.2.0 was rebuilt in an isolated temporary Cargo target. Extraction identity passed 6/6, the locked offline workspace build passed, all 19 tests passed, and five executions reproduced the same supported behavior and counterexamples. This local event is recorded in `evidence/10_release_candidate_validation_2026-08-24.txt`; independent public time begins only with an externally visible repository/archive record.

## Future independent publication

When authorized and complete, a release record should capture:

1. the exact allowlisted capsule;
2. the artifact hash manifest;
3. a signed version tag or equivalent immutable source reference;
4. an independent repository/archive timestamp;
5. DOI metadata if actually issued;
6. citation metadata matching the immutable artifact;
7. the selected license and provenance notice.

Do not add a DOI, repository URL, tag, or public-release date to this timeline until it exists and has been verified.
