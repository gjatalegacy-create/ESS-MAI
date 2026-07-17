/*
 * ╔══════════════════════════════════════════════════════════════════════════╗
 * ║  SHADOW_BUSS.H — ESS-MAI SHADOW PLATFORM                              ║
 * ║  Gjata Legacy™  |  Arkitekt: Bledar Gjata                             ║
 * ║                                                                          ║
 * ║  Bus i komunikimit Shadow — MPSC, zero-copy, C11.                     ║
 * ║  Mbart PassPackage nga Quantum → Shadow nodes.                        ║
 * ║  Mbart primitive_flags nga Light → shadow_gj_legacy.lgc               ║
 * ║                                                                          ║
 * ║  LIDHJET:                                                               ║
 * ║    ← Light Platform:   light_buss_send() → SB_TOPIC_PRIMITIVE         ║
 * ║    ← Quantum Platform: pim_shadow_bridge → SB_TOPIC_CLAIM             ║
 * ║    → Shadow nodes:     shadow_buss_recv() per secilin nod             ║
 * ╚══════════════════════════════════════════════════════════════════════════╝
 */

#ifndef SHADOW_BUSS_H
#define SHADOW_BUSS_H

#include <stdint.h>
#include <stddef.h>
#include <string.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ── CONSTANTS ──────────────────────────────────────────────────────────── */

#define SB_RING_N          128
#define SB_RING_MASK       127
#define SB_MAX_MODULE       64
#define SB_MAX_PAYLOAD    4096    /* PassPackage serialized max size         */
#define SB_PRIO_RINGS        3    /* LOW / HIGH / CRITICAL                   */

/* Prioritet */
#define SB_PRIO_LOW          0
#define SB_PRIO_HIGH         1
#define SB_PRIO_CRITICAL     2

/* Topics — nga cili kanal vjen mesazhi */
#define SB_TOPIC_CLAIM       0x01  /* Quantum PassPackage → Shadow           */
#define SB_TOPIC_PRIMITIVE   0x02  /* Light primitive_flags → shadow_gj      */
#define SB_TOPIC_INTERNAL    0x03  /* Shadow node → Shadow node              */

/* Return codes */
#define SB_OK                0
#define SB_ERR_NOT_INIT     -1
#define SB_ERR_FULL         -2
#define SB_ERR_EMPTY        -3
#define SB_ERR_PARAM        -6

/* ── 500 SEAL — KONSTANTJA SUPREME ─────────────────────────────────────── */

/* Vula primitive: (flags & 0x0000FFFF) ^ 0xA5A5 == 500              */
/* VETËM shadow_gj_legacy_check_seal() e interpreton si 500           */
/* Çdo nod tjetër e mbart verbërisht                                  */
#define SB_SEAL_XOR        0xA5A5U
#define SB_SEAL_MSK        0x0000FFFFU
#define SB_SEAL_TARGET     500U

/* ── STRUCTS ────────────────────────────────────────────────────────────── */

/*
 * ShadowBussMsg — mesazhi i plotë Shadow bus.
 * ABI-stable, #[repr(C)] Rust.
 */
typedef struct {
    char     from[SB_MAX_MODULE];      /* modul dërgues                      */
    char     to[SB_MAX_MODULE];        /* modul marrës (ose "SHADOW_GJ")     */
    uint8_t  payload[SB_MAX_PAYLOAD];  /* PassPackage i serializuar           */
    uint32_t payload_len;              /* gjatësia aktuale                    */
    uint32_t primitive_flags;          /* 0x0000_A4B1 ose 0 — mbartur verbër */
    uint8_t  topic;                    /* SB_TOPIC_*                          */
    uint8_t  priority;                 /* SB_PRIO_*                           */
    uint8_t  _pad[2];
    uint64_t ts_ns;                    /* timestamp ns                        */
    uint32_t seq;                      /* sekuencë monotonike                 */
    uint32_t crc32;                    /* integritet                          */
} ShadowBussMsg;

/* ── PUBLIC API ─────────────────────────────────────────────────────────── */

int  shadow_buss_init(void);
void shadow_buss_destroy(void);

int  shadow_buss_send(
    const char*    from,
    const char*    to,
    const uint8_t* payload,
    uint32_t       len,
    uint32_t       primitive_flags,   /* mbartur verbër — bus nuk e di       */
    uint8_t        topic,
    uint8_t        priority
);

int  shadow_buss_recv(
    const char*    target,
    ShadowBussMsg* msg_out
);

/*
 * shadow_gj_legacy_check_seal() — VETËM shadow_gj_legacy e thërret.
 * Kthim: 1 = 500 i pranishëm (PRIMITIVE), 0 = jo PRIMITIVE.
 */
int shadow_gj_legacy_check_seal(uint32_t primitive_flags);

#ifdef __cplusplus
}
#endif

#endif /* SHADOW_BUSS_H */
