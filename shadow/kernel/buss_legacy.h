/*
 * ╔══════════════════════════════════════════════════════════════════════════╗
 * ║  BUSS_LEGACY.H — BUS-I I VERBËR (C)                                   ║
 * ║  Gjata Legacy™  |  Arkitekt: Bledar Gjata                             ║
 * ║                                                                          ║
 * ║  Mbart vulën 500 nga Light → shadow_gj_legacy.                       ║
 * ║  I VERBËR: nuk e di ç'është 500, nuk e inspekton, as nuk e di që      ║
 * ║  po e mbart. Thjesht kopjon mesazhin → queue → recv.                  ║
 * ║                                                                          ║
 * ║  Ndarje pushtetesh: transporti ≠ autoriteti.                         ║
 * ╚══════════════════════════════════════════════════════════════════════════╝
 */

#ifndef BUSS_LEGACY_H
#define BUSS_LEGACY_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ── KONSTANTET ──────────────────────────────────────────────────────────── */

#define BL_QUEUE_N        128       /* slots — fuqi e 2                       */
#define BL_QUEUE_MASK     127       /* BL_QUEUE_N - 1                         */
#define BL_LGC_SEALED_LEN 512       /* string-u LGC i vulosur                 */
#define BL_MAX_PAYLOAD    2048

/* Kodet e kthimit */
#define BL_OK            0
#define BL_ERR_NOT_INIT -1
#define BL_ERR_FULL     -2
#define BL_ERR_EMPTY    -3
#define BL_ERR_PARAM    -6

/* ── MESAZHI ─────────────────────────────────────────────────────────────────
 * Bus-i nuk e shikon kurrë brendinë e 'flags'. E kopjon bit-për-bit.
 * ────────────────────────────────────────────────────────────────────────── */

typedef struct {
    uint64_t trace_id;                       /* origin_id nga EvolveTrace      */
    char     lgc_sealed[BL_LGC_SEALED_LEN];  /* string LGC i plotë             */
    uint32_t flags;                          /* mbart 0xA4XX (500) ose 0x0000  */
    uint8_t  payload[BL_MAX_PAYLOAD];        /* të dhënat e degës              */
    uint32_t payload_len;
    uint8_t  priority;
} BussLegacyMsg;

/* ── API PUBLIKE ─────────────────────────────────────────────────────────── */

/* Inicializon bus-in. Kthim: BL_OK. */
int  buss_legacy_init(void);

/*
 * Dërgon mesazhin në queue — KOPJON pikërisht siç është.
 * NUK inspekton msg->flags. NUK di ç'është 500.
 * Kthim: BL_OK, BL_ERR_FULL, BL_ERR_PARAM, BL_ERR_NOT_INIT.
 */
int  buss_legacy_send(const BussLegacyMsg *msg);

/*
 * Merr mesazhin e radhës (FIFO). Kopjon në msg_out.
 * Kthim: BL_OK, BL_ERR_EMPTY, BL_ERR_PARAM, BL_ERR_NOT_INIT.
 */
int  buss_legacy_recv(BussLegacyMsg *msg_out);

/* Numri i mesazheve aktualisht në queue. */
size_t buss_legacy_pending(void);

/* Pastron queue-në. */
void buss_legacy_flush(void);

#ifdef __cplusplus
}
#endif

#endif /* BUSS_LEGACY_H */
