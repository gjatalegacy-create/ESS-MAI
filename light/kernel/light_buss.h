/*
 * ╔══════════════════════════════════════════════════════════════════════════╗
 * ║  LIGHT_BUSS.H — ESS-MAI LIGHT PLATFORM                                ║
 * ║  Gjata Legacy™  |  Arkitekt: Bledar Gjata                             ║
 * ║                                                                          ║
 * ║  Bus i komunikimit për Light Platform.                                 ║
 * ║  4 nivele prioriteti — CRITICAL/HIGH/NORMAL/LOW.                      ║
 * ║  4 ring buffer të pavarur, mutex-protected, MPSC.                     ║
 * ║  CRC32 verifikim integriteti per çdo mesazh.                          ║
 * ║  LGC seal verifikim para çdo send.                                    ║
 * ╚══════════════════════════════════════════════════════════════════════════╝
 */

#ifndef LIGHT_BUSS_H
#define LIGHT_BUSS_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ── CONSTANTS ──────────────────────────────────────────────────────────── */

#define LB_RING_N         64       /* slots per ring — fuqi e 2              */
#define LB_RING_MASK      63       /* LB_RING_N - 1                          */
#define LB_MAX_MODULE     64       /* gjatësia max e emrit të modulit         */
#define LB_MAX_PAYLOAD    2048     /* payload max bytes per mesazh           */
#define LB_PRIO_RINGS     4        /* numri i ring-eve (1 per prioritet)     */

/* Nivelet e prioritetit */
#define LB_PRIO_LOW       0
#define LB_PRIO_NORMAL    1
#define LB_PRIO_HIGH      2
#define LB_PRIO_CRITICAL  3

/* Kodet e kthimit */
#define LB_OK             0
#define LB_ERR_NOT_INIT  -1
#define LB_ERR_FULL      -2
#define LB_ERR_EMPTY     -3
#define LB_ERR_CRC       -4
#define LB_ERR_SEAL      -5
#define LB_ERR_PARAM     -6
#define LB_ERR_OVERFLOW  -7

/* ── STRUCTS ────────────────────────────────────────────────────────────── */

/*
 * LightBussMsg — mesazhi i plotë.
 * Gjatësia totale: 64+64+2048+4+1+3+8+4+4 = 2200 bytes.
 * Padding explicit — ABI identike me Rust #[repr(C)].
 */
typedef struct {
    char     from[LB_MAX_MODULE];      /*  64 bytes — emri i modulit dërgues  */
    char     to[LB_MAX_MODULE];        /*  64 bytes — emri i modulit marrës   */
    uint8_t  payload[LB_MAX_PAYLOAD];  /* 2048 bytes — të dhënat               */
    uint32_t payload_len;              /*   4 bytes — gjatësia aktuale payload  */
    uint8_t  priority;                 /*   1 byte  — LB_PRIO_*               */
    uint8_t  _pad[3];                  /*   3 bytes — padding explicit         */
    uint64_t ts_ns;                    /*   8 bytes — timestamp ns             */
    uint32_t crc32;                    /*   4 bytes — CRC32 mbi të gjitha fushat */
    uint32_t _reserved;                /*   4 bytes — e rezervuar              */
} LightBussMsg;

/*
 * LightBussStats — statistikat e bus-it.
 * Lexohet nga Rust via light_buss_stats().
 */
typedef struct {
    uint64_t total_sent;        /* total mesazhe të dërguara                  */
    uint64_t total_recv;        /* total mesazhe të marra                     */
    uint64_t total_dropped;     /* mesazhe të humbura (ring plot)             */
    uint64_t crc_errors;        /* gabime CRC të detektuara                   */
    uint64_t seal_errors;       /* gabime LGC seal                            */
    uint64_t ring_pending[LB_PRIO_RINGS]; /* mesazhe në pritje per ring       */
} LightBussStats;

/* ── PUBLIC API ─────────────────────────────────────────────────────────── */

/*
 * light_buss_init() — inicializon bus-in.
 * Duhet thirrur 1 herë para çdo send/recv.
 * Kthim: LB_OK ose LB_ERR_NOT_INIT.
 */
int  light_buss_init(void);

/*
 * light_buss_send() — dërgon mesazh në bus.
 * from     — emri i modulit dërgues  (max LB_MAX_MODULE chars)
 * to       — emri i modulit marrës   (max LB_MAX_MODULE chars)
 * payload  — të dhënat               (max LB_MAX_PAYLOAD bytes)
 * len      — gjatësia e payload
 * priority — LB_PRIO_LOW..LB_PRIO_CRITICAL
 * Kthim: LB_OK, LB_ERR_FULL, LB_ERR_PARAM, LB_ERR_OVERFLOW.
 */
int  light_buss_send(
    const char*    from,
    const char*    to,
    const uint8_t* payload,
    uint32_t       len,
    uint8_t        priority
);

/*
 * light_buss_recv() — merr mesazhin e parë adresuar tek 'target'.
 * Kontrollon nga CRITICAL → LOW (prioritet i lartë më parë).
 * target  — emri i modulit marrës
 * msg_out — pointer ku ruhet mesazhi (i plotë)
 * Kthim: LB_OK, LB_ERR_EMPTY, LB_ERR_CRC.
 */
int  light_buss_recv(
    const char*   target,
    LightBussMsg* msg_out
);

/*
 * light_buss_stats() — statistikat aktuale.
 * stats_out duhet të jetë pointer valid tek LightBussStats.
 */
void light_buss_stats(LightBussStats* stats_out);

/*
 * light_buss_destroy() — pastron resurset e bus-it.
 */
void light_buss_destroy(void);

/*
 * light_buss_verify_seal() — verifikon LGC seal nga flags.
 * Logjika identike me shadow_lgc_seal_check():
 *   (flags & 0x0000FFFF) ^ 0xA5A5 == 500 → 1 (valid)
 *   çdo gjë tjetër → 0 (invalid)
 * Kthim: 1 = valid, 0 = invalid.
 */
int  light_buss_verify_seal(uint32_t flags);

#ifdef __cplusplus
}
#endif

#endif /* LIGHT_BUSS_H */
