/*
 * ╔══════════════════════════════════════════════════════════════════════════╗
 * ║  SHADOW_GJ_LEGACY.H — AUTORITETI SUPREM (C)                           ║
 * ║  Gjata Legacy™  |  Arkitekt: Bledar Gjata                             ║
 * ║                                                                          ║
 * ║  shadow_gj_legacy.lgc — i vetmi që di ç'është vula 500.              ║
 * ║  Merr LgcRequest nga bus (flags mbart 500 të fshehur me XOR).         ║
 * ║  Vendos 0/1. Nuk logohet. Nuk printohet.                             ║
 * ║                                                                          ║
 * ║  FORMAT FLAGS:  (flags & 0x0000FFFF) ^ 0xA5A5 == 500  → SEALED       ║
 * ║                 flags == 0  ose  != 500              → STRIPPED       ║
 * ╚══════════════════════════════════════════════════════════════════════════╝
 */

#ifndef SHADOW_GJ_LEGACY_H
#define SHADOW_GJ_LEGACY_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ── KONSTANTET E VULËS ──────────────────────────────────────────────────── */

#define SGL_SEAL_XOR     0xA5A5U      /* maska XOR e transportit               */
#define SGL_SEAL_MASK    0x0000FFFFU  /* 16 bit-et e poshtme mbartin vulën     */
#define SGL_SEAL_PRIMITIVE  500U      /* 5*100 — vula e primitivit të fortë    */

/* ── GJATËSITË E BUFER-AVE (identike me lgc_bridge.rs #[repr(C)]) ─────────── */

#define LGC_ID_LEN       48
#define LGC_MOD_NAME     64
#define LGC_SHA_LEN      65          /* 64 hex + NUL                          */
#define LGC_REASON_LEN  128

/* ── VENDIMET ────────────────────────────────────────────────────────────── */

typedef enum {
    LGC_DECISION_BLOCK  = 0,   /* 0 — STRIPPED / refuzuar                     */
    LGC_DECISION_PASS   = 1,   /* 1 — SEALED / pranuar si primitiv           */
    LGC_DECISION_REFINE = 2,
    LGC_DECISION_WAIT   = 3
} LgcDecision;

typedef enum {
    LGC_STATE_LIGHT     = 0,
    LGC_STATE_PRESSURED = 1,
    LGC_STATE_ABSORBING = 2,
    LGC_STATE_SUBMODULE = 3,
    LGC_STATE_FASTPATH  = 4
} LgcState;

/* ── STRUKTURAT (ABI identike me Rust #[repr(C)] në lgc_bridge.rs) ────────── */

typedef struct {
    uint8_t  req_id[LGC_ID_LEN];     /*  48 */
    uint8_t  caller[LGC_MOD_NAME];   /*  64 */
    uint8_t  sha256[LGC_SHA_LEN];    /*  65 */
    float    ram_usage;              /*   4 */
    uint32_t flags;                  /*   4 — vula 500 e fshehur këtu          */
    uint64_t ts;                     /*   8 */
} LgcRequest;

typedef struct {
    uint8_t     req_id[LGC_ID_LEN];     /*  48 */
    LgcDecision decision;               /*   4 */
    LgcState    state;                  /*   4 */
    uint8_t     sha256[LGC_SHA_LEN];    /*  65 */
    uint8_t     reason[LGC_REASON_LEN]; /* 128 */
    uint64_t    ts;                     /*   8 */
} LgcResult;

typedef struct {
    uint64_t total;
    uint64_t passed;
    uint64_t blocked;
    int32_t  state;
    int32_t  frozen;
} LgcStats;

/* ── API PUBLIKE ─────────────────────────────────────────────────────────── */

/* Inicializon autoritetin. Kthim: 0 = OK. */
int  lgc_init(void);

/*
 * Vendimi suprem 0/1 mbi një request.
 * Lexon req->flags → heq maskën → nëse == 500: PASS, ndryshe: BLOCK.
 * Shkruan res->decision, res->state, res->sha256 (echo), res->ts.
 * Kthim: 0 = procesuar.
 */
int  lgc_check(const LgcRequest *req, LgcResult *res);

/* SHA256 nga kerneli — out duhet [32] bytes. (delegohet te buss_legacy CRC-free) */
void lgc_sha256(const uint8_t *data, size_t len, uint8_t *out);

/* Statistikat agregate (total/passed/blocked). */
void lgc_get_stats(LgcStats *stats);

/* Ngrirje/shkrirje e fazave (sovereign control). */
int  lgc_freeze(int32_t stage);
int  lgc_unfreeze(void);

/*
 * shadow_lgc_seal_check() — primitivi i vulës, pa state.
 *   (flags & 0x0000FFFF) ^ 0xA5A5 == 500  → 1
 *   flags == 0  ose çdo gjë tjetër        → 0
 */
int  shadow_lgc_seal_check(uint32_t flags);

#ifdef __cplusplus
}
#endif

#endif /* SHADOW_GJ_LEGACY_H */
