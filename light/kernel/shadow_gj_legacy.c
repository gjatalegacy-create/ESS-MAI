/*
 * ╔══════════════════════════════════════════════════════════════════════════╗
 * ║  SHADOW_GJ_LEGACY.C — AUTORITETI SUPREM (C)                           ║
 * ║  Gjata Legacy™  |  Arkitekt: Bledar Gjata                             ║
 * ║                                                                          ║
 * ║  I VETMI që kupton vulën 500. Bus-i e mbart të verbër; këtu vendoset. ║
 * ║                                                                          ║
 * ║  LOGJIKA:                                                                ║
 * ║    flags → unseal (XOR 0xA5A5) → == 500 ?                            ║
 * ║      PO  → input mbeti PRIMITIV i fortë, nuk u fragmentua → PASS (1)  ║
 * ║      JO  → input u sfidua dhe u nda në gjendje/knowledge → BLOCK (0)  ║
 * ║                                                                          ║
 * ║  Nuk logohet. Nuk printohet. 0 ose 1.                                ║
 * ╚══════════════════════════════════════════════════════════════════════════╝
 */

/* POSIX feature-test për clock_gettime nën -std=c99 */
#if !defined(_POSIX_C_SOURCE) || (_POSIX_C_SOURCE < 199309L)
#define _POSIX_C_SOURCE 199309L
#endif

#include "shadow_gj_legacy.h"
#include <string.h>
#include <time.h>

/* ── GJENDJE GLOBALE — atomike-lite (single authority, MPSC nga bus) ──────── */

static int      g_initialized = 0;
static uint64_t g_total       = 0;
static uint64_t g_passed      = 0;
static uint64_t g_blocked     = 0;
static int32_t  g_frozen      = 0;

/* ── §1  PRIMITIVI I VULËS — pa state, pure ──────────────────────────────── */

/*
 * Heq maskën e transportit dhe krahason me vulën e primitivit.
 * E ndarë qëllimisht: i njëjti primitiv përdoret nga light_buss.c.
 */
static uint32_t sgl_unseal(uint32_t flags) {
    return (flags & SGL_SEAL_MASK) ^ SGL_SEAL_XOR;
}

int shadow_lgc_seal_check(uint32_t flags) {
    /* flags == 0 → vula u hoq gjatë evolve_trace (fragmentim) → 0 */
    uint32_t cleared = (uint32_t)(flags != 0);
    uint32_t value   = sgl_unseal(flags);
    /* (flags!=0) AND (value==500) — pa degëzim semantik */
    return (int)(cleared & (uint32_t)(value == SGL_SEAL_PRIMITIVE));
}

/* ── §2  TS NS ───────────────────────────────────────────────────────────── */

static uint64_t sgl_ts_ns(void) {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (uint64_t)ts.tv_sec * 1000000000ULL + (uint64_t)ts.tv_nsec;
}

/* ── §3  INIT ────────────────────────────────────────────────────────────── */

int lgc_init(void) {
    g_initialized = 1;
    g_total = g_passed = g_blocked = 0;
    g_frozen = 0;
    return 0;
}

/* ── §4  VENDIMI SUPREM 0/1 ──────────────────────────────────────────────── */

int lgc_check(const LgcRequest *req, LgcResult *res) {
    if (req == NULL || res == NULL) {
        return -1;
    }

    /* Echo i identitetit (req_id + sha256) → res, zero interpretim */
    memcpy(res->req_id, req->req_id, LGC_ID_LEN);
    memcpy(res->sha256, req->sha256, LGC_SHA_LEN);
    res->reason[0] = '\0';            /* zero reason — nuk logohet */
    res->ts        = sgl_ts_ns();

    /* Nëse i ngrirë → bllokon gjithçka (sovereign freeze) */
    uint32_t live   = (uint32_t)(g_frozen == 0);

    /* Vula: 1 nëse primitivi mbijetoi, 0 nëse u fragmentua */
    uint32_t sealed = (uint32_t)shadow_lgc_seal_check(req->flags);

    uint32_t pass   = live & sealed;

    res->decision = pass ? LGC_DECISION_PASS : LGC_DECISION_BLOCK;

    /* State: nën presion nëse RAM e lartë, ndryshe LIGHT (informativ) */
    res->state = (req->ram_usage > 0.50f)
                 ? LGC_STATE_PRESSURED
                 : LGC_STATE_LIGHT;

    /* Statistika */
    g_total  += 1;
    g_passed += pass;
    g_blocked += (1u - pass);

    return 0;
}

/* ── §5  SHA256 placeholder ──────────────────────────────────────────────────
 * Shadow-i real do mbartë SHA256 të vërtetë; këtu kerneli C i Light-it nuk e
 * llogarit (kompetencë e Shadow Platform). Mbushim me 0 për ABI-siguri.
 * Rust-i përdor sha2 crate kur i duhet hash real (lgc_bridge::sha256_internal).
 * ────────────────────────────────────────────────────────────────────────── */

/* ⚠️ PLACEHOLDER SHA — GATE-UAR JASHTË PRODHIMIT (#9, v1.0.3).
 *
 * Ky funksion NUK llogarit SHA-256 real; mbushte me zero. Tani është gate-uar:
 *   • Në PRODHIM (default): NUK kompilohet. Çdo thirrje → link error → detyron Rust.
 *   • Vetëm me -DSOVEREIGN_ALLOW_PLACEHOLDER_SHA (testim/verify_kernel) → stub.
 *
 * Rruga Rust (sovereign/sha256.rs::sha256_hex) është autoriteti i VETËM kriptografik.
 * lgc_bridge.rs::sha256_of() NUK e thërret këtë (hequr në v1.0.2).
 * Implemento SHA-256 real PARA se ndonjë rrugë prodhimi ta përdorë. */
#ifdef SOVEREIGN_ALLOW_PLACEHOLDER_SHA
void lgc_sha256(const uint8_t *data, size_t len, uint8_t *out) {
    (void)data;
    (void)len;
    if (out != NULL) {
        memset(out, 0, 32);  /* PLACEHOLDER — jo hash real, vetëm jashtë prodhimit */
    }
}
#else
/* PRODHIM: lgc_sha256 qëllimisht MUNGON. Përdor Rust sha256_hex.
 * Nëse ndonjë rrugë C e thërret pa flag-un, link-u dështon → fail-closed. */
#endif

/* ── §6  STATS / FREEZE ──────────────────────────────────────────────────── */

void lgc_get_stats(LgcStats *stats) {
    if (stats == NULL) {
        return;
    }
    stats->total   = g_total;
    stats->passed  = g_passed;
    stats->blocked = g_blocked;
    stats->state   = 0;
    stats->frozen  = g_frozen;
}

int lgc_freeze(int32_t stage) {
    g_frozen = (stage != 0) ? stage : 1;
    return 0;
}

int lgc_unfreeze(void) {
    g_frozen = 0;
    return 0;
}
