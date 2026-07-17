/*
 * ╔══════════════════════════════════════════════════════════════════════════╗
 * ║  SHADOW_BUSS.C — ESS-MAI SHADOW PLATFORM                              ║
 * ║  Gjata Legacy™  |  Arkitekt: Bledar Gjata                             ║
 * ║                                                                          ║
 * ║  MPSC ring buffer — 3 prioritete — mutex-protected.                   ║
 * ║  Zero malloc — statik, deterministic.                                  ║
 * ║  CRC32 verifikim për çdo mesazh.                                      ║
 * ╚══════════════════════════════════════════════════════════════════════════╝
 */

#include "shadow_buss.h"
#include <pthread.h>
#include <stdatomic.h>
#include <stdio.h>

/* ── CRC32 TABLE ────────────────────────────────────────────────────────── */

static uint32_t crc32_table[256];
static atomic_int crc32_ready = ATOMIC_VAR_INIT(0);

static void crc32_init(void) {
    if (atomic_load(&crc32_ready)) return;
    for (uint32_t i = 0; i < 256; i++) {
        uint32_t c = i;
        for (int j = 0; j < 8; j++)
            c = (c & 1) ? (0xEDB88320U ^ (c >> 1)) : (c >> 1);
        crc32_table[i] = c;
    }
    atomic_store(&crc32_ready, 1);
}

static uint32_t crc32_compute(const uint8_t* data, size_t len) {
    uint32_t c = 0xFFFFFFFFU;
    for (size_t i = 0; i < len; i++)
        c = crc32_table[(c ^ data[i]) & 0xFF] ^ (c >> 8);
    return c ^ 0xFFFFFFFFU;
}

/* ── RING BUFFER ────────────────────────────────────────────────────────── */

typedef struct {
    ShadowBussMsg slots[SB_RING_N];
    volatile uint32_t head;
    volatile uint32_t tail;
    pthread_mutex_t   lock;
} SbRing;

static SbRing  s_rings[SB_PRIO_RINGS];
static atomic_int s_init = ATOMIC_VAR_INIT(0);
static atomic_uint_fast32_t s_seq = ATOMIC_VAR_INIT(0);

/* ── INIT / DESTROY ─────────────────────────────────────────────────────── */

int shadow_buss_init(void) {
    if (atomic_load(&s_init)) return SB_OK;
    crc32_init();
    for (int p = 0; p < SB_PRIO_RINGS; p++) {
        s_rings[p].head = 0;
        s_rings[p].tail = 0;
        if (pthread_mutex_init(&s_rings[p].lock, NULL) != 0)
            return SB_ERR_NOT_INIT;
    }
    atomic_store(&s_init, 1);
    return SB_OK;
}

void shadow_buss_destroy(void) {
    if (!atomic_load(&s_init)) return;
    for (int p = 0; p < SB_PRIO_RINGS; p++)
        pthread_mutex_destroy(&s_rings[p].lock);
    atomic_store(&s_init, 0);
}

/* ── SEND ───────────────────────────────────────────────────────────────── */

int shadow_buss_send(
    const char*    from,
    const char*    to,
    const uint8_t* payload,
    uint32_t       len,
    uint32_t       primitive_flags,
    uint8_t        topic,
    uint8_t        priority
) {
    if (!atomic_load(&s_init))   return SB_ERR_NOT_INIT;
    if (!from || !to || !payload) return SB_ERR_PARAM;
    if (len > SB_MAX_PAYLOAD)    return SB_ERR_PARAM;
    if (priority >= SB_PRIO_RINGS) return SB_ERR_PARAM;

    SbRing* ring = &s_rings[priority];

    pthread_mutex_lock(&ring->lock);

    uint32_t next = (ring->head + 1) & SB_RING_MASK;
    if (next == ring->tail) {
        pthread_mutex_unlock(&ring->lock);
        return SB_ERR_FULL;
    }

    ShadowBussMsg* m = &ring->slots[ring->head];
    memset(m, 0, sizeof(*m));

    strncpy(m->from, from, SB_MAX_MODULE - 1);
    strncpy(m->to,   to,   SB_MAX_MODULE - 1);
    memcpy(m->payload, payload, len);
    m->payload_len     = len;
    m->primitive_flags = primitive_flags;  /* mbartur verbër — pa inspektim  */
    m->topic           = topic;
    m->priority        = priority;
    m->ts_ns           = 0;               /* platforma e vendos              */
    m->seq             = atomic_fetch_add(&s_seq, 1);

    /* CRC32 mbi gjithçka përveç crc32 fushës */
    m->crc32 = crc32_compute((const uint8_t*)m,
                              offsetof(ShadowBussMsg, crc32));

    ring->head = next;
    pthread_mutex_unlock(&ring->lock);
    return SB_OK;
}

/* ── RECV ───────────────────────────────────────────────────────────────── */

int shadow_buss_recv(const char* target, ShadowBussMsg* msg_out) {
    if (!atomic_load(&s_init) || !target || !msg_out)
        return SB_ERR_NOT_INIT;

    /* CRITICAL → HIGH → LOW — prioritet i lartë zgjidhet i pari */
    for (int p = SB_PRIO_RINGS - 1; p >= 0; p--) {
        SbRing* ring = &s_rings[p];
        pthread_mutex_lock(&ring->lock);

        uint32_t cur = ring->tail;
        while (cur != ring->head) {
            ShadowBussMsg* m = &ring->slots[cur];
            if (strncmp(m->to, target, SB_MAX_MODULE) == 0) {
                /* CRC verifikim */
                uint32_t expected = crc32_compute(
                    (const uint8_t*)m, offsetof(ShadowBussMsg, crc32));
                if (expected != m->crc32) {
                    pthread_mutex_unlock(&ring->lock);
                    return SB_ERR_PARAM;
                }
                *msg_out = *m;
                /* Zhvendos mbetjet */
                uint32_t i = cur;
                while (i != ring->head) {
                    uint32_t nxt = (i + 1) & SB_RING_MASK;
                    ring->slots[i] = ring->slots[nxt];
                    i = nxt;
                }
                ring->head = (ring->head - 1) & SB_RING_MASK;
                pthread_mutex_unlock(&ring->lock);
                return SB_OK;
            }
            cur = (cur + 1) & SB_RING_MASK;
        }
        pthread_mutex_unlock(&ring->lock);
    }
    return SB_ERR_EMPTY;
}

/* ── 500 SEAL — VETËM shadow_gj_legacy ─────────────────────────────────── */

/*
 * shadow_gj_legacy_check_seal()
 *
 * Ky funksion thirret VETËM nga shadow_gj_legacy_kernel.c
 * Asnjë nod tjetër nuk e di se çfarë do të thotë 500.
 *
 * Logjika:
 *   (primitive_flags & 0x0000FFFF) ^ 0xA5A5 == 500
 *   → 1: inputi mbijetoi i padividuar — PRIMITIVE
 *   → 0: inputi u nda — jo primitiv
 */
int shadow_gj_legacy_check_seal(uint32_t primitive_flags) {
    uint32_t seal = (primitive_flags & SB_SEAL_MSK) ^ SB_SEAL_XOR;
    /* Zero if/else — aritmetikë e pastër */
    return (int)(seal == SB_SEAL_TARGET);
}
