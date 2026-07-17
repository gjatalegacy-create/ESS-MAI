#define _POSIX_C_SOURCE 200809L
/*
 * ╔══════════════════════════════════════════════════════════════════════════╗
 * ║  LIGHT_BUSS.C — ESS-MAI LIGHT PLATFORM                                ║
 * ║  Gjata Legacy™  |  Arkitekt: Bledar Gjata                             ║
 * ║                                                                          ║
 * ║  Implementim real ekzekutiv i bus-it të Light Platform.              ║
 * ║                                                                          ║
 * ║  §1  CRC32 — standard IEEE 802.3 polynomial 0xEDB88320               ║
 * ║  §2  Timestamp — clock_gettime(CLOCK_REALTIME)                        ║
 * ║  §3  PrioQueue — ring buffer mutex-protected per prioritet            ║
 * ║  §4  Gjendje globale — 4 ring buffer + statistika atomike             ║
 * ║  §5  LGC Seal — (flags & 0xFFFF) ^ 0xA5A5 == 500                    ║
 * ║  §6  Operacionet e ring buffer (push/pop/count)                       ║
 * ║  §7  API publike (init/send/recv/stats/destroy/verify_seal)           ║
 * ╚══════════════════════════════════════════════════════════════════════════╝
 */

#include "light_buss.h"

#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <pthread.h>

/* ════════════════════════════════════════════════════════════════════════════
 * §1  CRC32 — IEEE 802.3, polynomial 0xEDB88320 (reflected)
 *     Zero lookup table — branchless xor+shift
 * ════════════════════════════════════════════════════════════════════════════ */

static uint32_t crc32_compute(const uint8_t *data, size_t len) {
    uint32_t crc = 0xFFFFFFFFu;
    size_t i;
    int j;
    for (i = 0; i < len; i++) {
        crc ^= (uint32_t)data[i];
        for (j = 0; j < 8; j++) {
            uint32_t mask = (uint32_t)(-(int32_t)(crc & 1u));
            crc = (crc >> 1u) ^ (0xEDB88320u & mask);
        }
    }
    return ~crc;
}

/*
 * Llogarit CRC32 mbi gjithë fushat e LightBussMsg
 * përveç fushës crc32 dhe _reserved.
 * Zona: bytes [0 .. sizeof(msg) - 8)
 */
static uint32_t msg_crc(const LightBussMsg *msg) {
    /* Llogarit CRC mbi: from + to + payload + payload_len + priority + _pad + ts_ns */
    /* Shmang 8 bytes të fundit (crc32 + _reserved) */
    size_t cover = sizeof(LightBussMsg) - 8u;
    return crc32_compute((const uint8_t *)msg, cover);
}

/* ════════════════════════════════════════════════════════════════════════════
 * §2  TIMESTAMP — nanoseconds nga UNIX_EPOCH
 * ════════════════════════════════════════════════════════════════════════════ */

static uint64_t ts_now_ns(void) {
    struct timespec ts;
    if (clock_gettime(CLOCK_REALTIME, &ts) != 0) { return 0ULL; }
    return (uint64_t)ts.tv_sec * 1000000000ULL + (uint64_t)ts.tv_nsec;
}

/* ════════════════════════════════════════════════════════════════════════════
 * §3  PRIO QUEUE — ring buffer me mutex
 *     MPSC (multi-producer, single consumer) per çdo prioritet
 * ════════════════════════════════════════════════════════════════════════════ */

typedef struct {
    LightBussMsg  slots[LB_RING_N]; /* slotet e mesazheve                    */
    uint64_t      head;             /* pozicioni i radhës (shkrim)           */
    uint64_t      tail;             /* pozicioni i leximit                   */
    pthread_mutex_t mutex;          /* mutex për MPSC                        */
} PrioQueue;

/* Kthim: numri i mesazheve aktualisht në ring */
static uint64_t pq_count(const PrioQueue *q) {
    return q->head - q->tail;
}

/* Shton mesazh në ring — kthehet 1 nëse ok, 0 nëse plot */
static int pq_push(PrioQueue *q, const LightBussMsg *msg) {
    if (pq_count(q) >= (uint64_t)LB_RING_N) { return 0; }
    uint64_t pos = q->head & (uint64_t)LB_RING_MASK;
    memcpy(&q->slots[pos], msg, sizeof(LightBussMsg));
    q->head++;
    return 1;
}

/*
 * Kërkon mesazhin e parë për 'target' nga pozicioni aktual.
 * Skanon nga tail deri te head — O(N).
 * Nëse gjendet: kopjon në msg_out, e largon nga ring (compaction).
 * Kthim: 1 nëse gjeti, 0 nëse empty ose nuk ka mesazh për target.
 */
static int pq_pop_for(PrioQueue *q, const char *target, LightBussMsg *msg_out) {
    uint64_t i;
    for (i = q->tail; i < q->head; i++) {
        uint64_t pos = i & (uint64_t)LB_RING_MASK;
        if (strncmp(q->slots[pos].to, target, LB_MAX_MODULE - 1u) == 0) {
            /* Kopjo mesazhin */
            memcpy(msg_out, &q->slots[pos], sizeof(LightBussMsg));
            /* Mbyll boshllëkun — zhvendos nga i mesazhi drejt tail */
            uint64_t j;
            for (j = i; j > q->tail; j--) {
                uint64_t cur  = j  & (uint64_t)LB_RING_MASK;
                uint64_t prev = (j - 1u) & (uint64_t)LB_RING_MASK;
                memcpy(&q->slots[cur], &q->slots[prev], sizeof(LightBussMsg));
            }
            q->tail++;
            return 1;
        }
    }
    return 0;
}

/* ════════════════════════════════════════════════════════════════════════════
 * §4  GJENDJE GLOBALE
 * ════════════════════════════════════════════════════════════════════════════ */

typedef struct {
    PrioQueue rings[LB_PRIO_RINGS]; /* ring[0]=LOW .. ring[3]=CRITICAL      */
    uint64_t  total_sent;
    uint64_t  total_recv;
    uint64_t  total_dropped;
    uint64_t  crc_errors;
    uint64_t  seal_errors;
    int       initialized;
    pthread_mutex_t stats_mutex;    /* mutex për statistikat globale         */
} LightBussState;

static LightBussState g_state;

/* ════════════════════════════════════════════════════════════════════════════
 * §5  LGC SEAL
 *     (flags & 0x0000FFFF) ^ 0xA5A5 == 500 → 1 (valid)
 *     Identike me shadow_gj_legacy_seal.c
 * ════════════════════════════════════════════════════════════════════════════ */

int light_buss_verify_seal(uint32_t flags) {
    if (flags == 0u) { return 0; }
    uint32_t v = (flags & 0x0000FFFFu) ^ 0xA5A5u;
    return (v == 500u) ? 1 : 0;
}

/* ════════════════════════════════════════════════════════════════════════════
 * §6  STRING UTILS
 * ════════════════════════════════════════════════════════════════════════════ */

static void lb_strncpy(char *dst, const char *src, size_t n) {
    size_t i = 0;
    if (src == NULL || dst == NULL || n == 0) { return; }
    for (; i < n - 1u && src[i] != '\0'; i++) { dst[i] = src[i]; }
    dst[i] = '\0';
}

static int lb_str_empty(const char *s) {
    return (s == NULL || s[0] == '\0');
}

/* ════════════════════════════════════════════════════════════════════════════
 * §7  API PUBLIKE
 * ════════════════════════════════════════════════════════════════════════════ */

int light_buss_init(void) {
    int r;
    int i;

    if (g_state.initialized) { return LB_OK; }

    memset(&g_state, 0, sizeof(LightBussState));

    /* Inicializo mutex-et e ring-eve */
    for (i = 0; i < LB_PRIO_RINGS; i++) {
        r = pthread_mutex_init(&g_state.rings[i].mutex, NULL);
        if (r != 0) { return LB_ERR_NOT_INIT; }
    }

    /* Inicializo mutex-in e statistikave */
    r = pthread_mutex_init(&g_state.stats_mutex, NULL);
    if (r != 0) { return LB_ERR_NOT_INIT; }

    g_state.initialized = 1;
    return LB_OK;
}

/* ─────────────────────────────────────────────────────────────────────────── */

int light_buss_send(
    const char*    from,
    const char*    to,
    const uint8_t* payload,
    uint32_t       len,
    uint8_t        priority
) {
    LightBussMsg msg;
    PrioQueue   *ring;
    int          ring_idx;
    int          pushed;

    if (!g_state.initialized)              { return LB_ERR_NOT_INIT; }
    if (lb_str_empty(from))                { return LB_ERR_PARAM;    }
    if (lb_str_empty(to))                  { return LB_ERR_PARAM;    }
    if (payload == NULL && len > 0u)       { return LB_ERR_PARAM;    }
    if (len > (uint32_t)LB_MAX_PAYLOAD)    { return LB_ERR_OVERFLOW; }
    if (priority >= (uint8_t)LB_PRIO_RINGS){ return LB_ERR_PARAM;   }

    /* Ndërto mesazhin */
    memset(&msg, 0, sizeof(LightBussMsg));
    lb_strncpy(msg.from, from, LB_MAX_MODULE);
    lb_strncpy(msg.to,   to,   LB_MAX_MODULE);

    if (payload != NULL && len > 0u) {
        memcpy(msg.payload, payload, (size_t)len);
    }
    msg.payload_len = len;
    msg.priority    = priority;
    memset(msg._pad, 0, sizeof(msg._pad));
    msg.ts_ns       = ts_now_ns();
    msg._reserved   = 0u;

    /* CRC32 mbi të gjitha fushat para crc+reserved */
    msg.crc32 = msg_crc(&msg);

    /* Dërgo në ring-un e duhur */
    ring_idx = (int)priority;
    ring     = &g_state.rings[ring_idx];

    pthread_mutex_lock(&ring->mutex);
    pushed = pq_push(ring, &msg);
    pthread_mutex_unlock(&ring->mutex);

    /* Statistika */
    pthread_mutex_lock(&g_state.stats_mutex);
    if (pushed) { g_state.total_sent++;   }
    else        { g_state.total_dropped++; }
    pthread_mutex_unlock(&g_state.stats_mutex);

    return pushed ? LB_OK : LB_ERR_FULL;
}

/* ─────────────────────────────────────────────────────────────────────────── */

int light_buss_recv(
    const char*   target,
    LightBussMsg* msg_out
) {
    int i;
    int found;
    uint32_t expected_crc;

    if (!g_state.initialized)   { return LB_ERR_NOT_INIT; }
    if (lb_str_empty(target))   { return LB_ERR_PARAM;    }
    if (msg_out == NULL)        { return LB_ERR_PARAM;    }

    /* Kontrollo nga CRITICAL (3) → LOW (0) */
    for (i = LB_PRIO_RINGS - 1; i >= 0; i--) {
        PrioQueue *ring = &g_state.rings[i];

        pthread_mutex_lock(&ring->mutex);
        found = pq_pop_for(ring, target, msg_out);
        pthread_mutex_unlock(&ring->mutex);

        if (found) {
            /* Verifiko CRC */
            expected_crc = msg_crc(msg_out);
            if (msg_out->crc32 != expected_crc) {
                pthread_mutex_lock(&g_state.stats_mutex);
                g_state.crc_errors++;
                pthread_mutex_unlock(&g_state.stats_mutex);
                return LB_ERR_CRC;
            }

            /* Statistika */
            pthread_mutex_lock(&g_state.stats_mutex);
            g_state.total_recv++;
            pthread_mutex_unlock(&g_state.stats_mutex);

            return LB_OK;
        }
    }

    return LB_ERR_EMPTY;
}

/* ─────────────────────────────────────────────────────────────────────────── */

void light_buss_stats(LightBussStats* stats_out) {
    int i;

    if (stats_out == NULL) { return; }
    memset(stats_out, 0, sizeof(LightBussStats));

    if (!g_state.initialized) { return; }

    pthread_mutex_lock(&g_state.stats_mutex);
    stats_out->total_sent    = g_state.total_sent;
    stats_out->total_recv    = g_state.total_recv;
    stats_out->total_dropped = g_state.total_dropped;
    stats_out->crc_errors    = g_state.crc_errors;
    stats_out->seal_errors   = g_state.seal_errors;
    pthread_mutex_unlock(&g_state.stats_mutex);

    /* Mesazhet në pritje per ring */
    for (i = 0; i < LB_PRIO_RINGS; i++) {
        pthread_mutex_lock(&g_state.rings[i].mutex);
        stats_out->ring_pending[i] = pq_count(&g_state.rings[i]);
        pthread_mutex_unlock(&g_state.rings[i].mutex);
    }
}

/* ─────────────────────────────────────────────────────────────────────────── */

void light_buss_destroy(void) {
    int i;

    if (!g_state.initialized) { return; }

    for (i = 0; i < LB_PRIO_RINGS; i++) {
        pthread_mutex_destroy(&g_state.rings[i].mutex);
    }
    pthread_mutex_destroy(&g_state.stats_mutex);

    memset(&g_state, 0, sizeof(LightBussState));
}
