/*
 * ╔══════════════════════════════════════════════════════════════════════════╗
 * ║  SHADOW_GJ_LEGACY_KERNEL.C — ESS-MAI SHADOW PLATFORM                  ║
 * ║  Gjata Legacy™  |  Arkitekt: Bledar Gjata                             ║
 * ║                                                                          ║
 * ║  Autoriteti suprem i sistemit.                                         ║
 * ║  I VETMI që shikon 500 seal.                                           ║
 * ║  I VETMI që shkruan në KnowledgeVault (via Rust callback).            ║
 * ║                                                                          ║
 * ║  LGC LAW — 5 ligjet sovrane:                                          ║
 * ║    LAW_1: PRIMITIVE = input i padividuar + 500 seal                   ║
 * ║    LAW_2: VERIFIED  = input i vërtetë, 500 humbi                      ║
 * ║    LAW_3: PRIMITIVE → akumulon pikë → LEGACY_KNOWLEDGE path           ║
 * ║    LAW_4: LEGACY    = i paprekshëm — Shadow nuk e modifikon           ║
 * ║    LAW_5: NEGATIVE  = dije e vlefshme — jo gabim, dije e verifikuar  ║
 * ╚══════════════════════════════════════════════════════════════════════════╝
 */

#include "shadow_buss.h"
#include <stdint.h>
#include <string.h>

/* ── CALLBACKS → Rust (KnowledgeVault) ─────────────────────────────────── */

/*
 * Funksionet e mëposhtme implementohen në Rust (shadow_gj_legacy.rs)
 * dhe thirren nga ky kernel C.
 * Deklarime extern — lidhja ndodh gjatë kompilimit.
 */
extern void vault_write_primitive(
    const uint8_t* pkg_bytes,
    uint32_t       pkg_len,
    uint64_t       ts_ns
);

extern void vault_write_verified(
    const uint8_t* pkg_bytes,
    uint32_t       pkg_len,
    uint8_t        knowledge_band,  /* 0=negative,1=hypothesis,2=fact,3=sci  */
    uint64_t       ts_ns
);

extern void vault_write_negative(
    const uint8_t* pkg_bytes,
    uint32_t       pkg_len,
    uint64_t       ts_ns
);

/* ── SUPREME VERDICT STRUCT ─────────────────────────────────────────────── */

typedef struct {
    uint8_t  verified;      /* 0 ose 1 — judiciary verdict                  */
    uint8_t  primitive;     /* 0 ose 1 — 500 seal + verified                */
    uint8_t  knowledge_band;/* 0=neg 1=hyp 2=fact 3=sci                     */
    uint8_t  _pad;
    uint32_t lgc_law;       /* bitmask ligjeve të aplikuara                 */
    float    legacy_score;  /* score akumulativ drejt LEGACY                */
} SupremeVerdict;

/* ── LGC LAW BITMASK ────────────────────────────────────────────────────── */

#define LGC_LAW_1_PRIMITIVE   (1u << 0)   /* inputi është PRIMITIVE          */
#define LGC_LAW_2_VERIFIED    (1u << 1)   /* inputi është VERIFIED           */
#define LGC_LAW_3_LEGACY_PATH (1u << 2)   /* fillon rruga → LEGACY           */
#define LGC_LAW_4_IMMUTABLE   (1u << 3)   /* LEGACY = i paprekshëm           */
#define LGC_LAW_5_NEGATIVE    (1u << 4)   /* NEGATIVE = dije e vlefshme      */

/* ── SCORE VEKTORIAL — ZERO IF/ELSE ────────────────────────────────────── */

/*
 * legacy_score_compute — sa afër LEGACY është ky primitiv.
 * Input: normalized scores [0.0-1.0]
 * Output: [0.0-1.0] — mbi 0.75 → LEGACY path aktiv
 *
 * Formula: vektor i peshuar, zero branching.
 */
static float legacy_score_compute(
    float evidence_density,
    float logical_coherence,
    float causal_integrity,
    float convergence_strength,
    float reproducibility
) {
    /* Peshat direkt nga teoria ESS-MAI */
    const float w0 = 0.25f;  /* evidence_density      */
    const float w1 = 0.20f;  /* logical_coherence     */
    const float w2 = 0.25f;  /* causal_integrity      */
    const float w3 = 0.15f;  /* convergence_strength  */
    const float w4 = 0.15f;  /* reproducibility       */

    float score =
        evidence_density     * w0 +
        logical_coherence    * w1 +
        causal_integrity     * w2 +
        convergence_strength * w3 +
        reproducibility      * w4;

    /* Clamp [0.0-1.0] — zero if/else */
    score = score < 0.0f ? 0.0f : score;
    score = score > 1.0f ? 1.0f : score;
    return score;
}

/* ── SUPREME JUDGE — ENTRY POINT ────────────────────────────────────────── */

/*
 * shadow_gj_legacy_judge()
 *
 * Thirret nga Rust (shadow_gj_legacy.rs) pas S.Judiciary verdict.
 *
 * Parametrat:
 *   primitive_flags  — 0x0000_A4B1 (500) ose 0x0000_0000
 *   judiciary_ok     — 0/1 nga S.Judiciary
 *   knowledge_band   — 0=neg 1=hyp 2=fact 3=sci
 *   pkg_bytes        — PassPackage i serializuar
 *   pkg_len          — gjatësia
 *   ts_ns            — timestamp
 *   ev_density..repr — 5 score nga PotentialityProfile
 *
 * Kthim: SupremeVerdict
 */
SupremeVerdict shadow_gj_legacy_judge(
    uint32_t       primitive_flags,
    uint8_t        judiciary_ok,
    uint8_t        knowledge_band,
    const uint8_t* pkg_bytes,
    uint32_t       pkg_len,
    uint64_t       ts_ns,
    float          ev_density,
    float          log_coherence,
    float          caus_integrity,
    float          conv_strength,
    float          reproducibility
) {
    SupremeVerdict v;
    memset(&v, 0, sizeof(v));

    /* ── HAPI 1: shiko 500 — VETËM KËTU ─────────────────────────────── */
    int has_500 = shadow_gj_legacy_check_seal(primitive_flags);

    /* ── HAPI 2: llogarit legacy_score (zero if/else) ────────────────── */
    float lscore = legacy_score_compute(
        ev_density, log_coherence, caus_integrity,
        conv_strength, reproducibility
    );

    /* ── HAPI 3: verified dhe primitive — aritmetikë e pastër ─────────── */
    v.verified       = judiciary_ok;
    v.primitive      = (uint8_t)(judiciary_ok & (uint8_t)has_500);
    v.knowledge_band = knowledge_band;
    v.legacy_score   = lscore;

    /* ── HAPI 4: LGC LAW bitmask ─────────────────────────────────────── */
    /*
     * Bitmask vektorial — zero if/else:
     *   primitive=1 → LAW_1 + LAW_2 + LAW_3
     *   verified=1, primitive=0 → LAW_2
     *   verified=0 → LAW_5
     */
    uint32_t law = 0;

    /* LAW_1: PRIMITIVE — (verified & has_500) */
    law |= (uint32_t)(v.primitive) * LGC_LAW_1_PRIMITIVE;

    /* LAW_2: VERIFIED — judiciary_ok */
    law |= (uint32_t)(v.verified)  * LGC_LAW_2_VERIFIED;

    /* LAW_3: LEGACY PATH — primitive + legacy_score > 0.50 */
    uint32_t legacy_active = (uint32_t)(v.primitive)
                           & (uint32_t)(lscore > 0.50f);
    law |= legacy_active * LGC_LAW_3_LEGACY_PATH;

    /* LAW_5: NEGATIVE — jo verified */
    uint32_t is_negative = (uint32_t)(1u - (uint32_t)v.verified);
    law |= is_negative * LGC_LAW_5_NEGATIVE;

    v.lgc_law = law;

    /* ── HAPI 5: shkruaj në KnowledgeVault (via Rust callback) ──────── */
    if (v.primitive) {
        /* LAW_1 — PrimitiveStore */
        vault_write_primitive(pkg_bytes, pkg_len, ts_ns);
    } else if (v.verified) {
        /* LAW_2 — sipas knowledge_band */
        vault_write_verified(pkg_bytes, pkg_len, knowledge_band, ts_ns);
    } else {
        /* LAW_5 — NegativeStore (dije e vlefshme) */
        vault_write_negative(pkg_bytes, pkg_len, ts_ns);
    }

    return v;
}
