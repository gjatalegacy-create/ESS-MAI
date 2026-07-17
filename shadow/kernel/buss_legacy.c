/*
 * ╔══════════════════════════════════════════════════════════════════════════╗
 * ║  BUSS_LEGACY.C — BUS-I I VERBËR (C)                                   ║
 * ║  Gjata Legacy™  |  Arkitekt: Bledar Gjata                             ║
 * ║                                                                          ║
 * ║  FIFO mutex-protected. Kopjon BussLegacyMsg pikërisht siç është.      ║
 * ║  Kurrë nuk lexon msg->flags. Kurrë nuk vendos. Vetëm transporton.     ║
 * ╚══════════════════════════════════════════════════════════════════════════╝
 */

#include "buss_legacy.h"
#include <string.h>
#include <pthread.h>

/* ── QUEUE GLOBALE — ring FIFO, mutex-protected ──────────────────────────── */

typedef struct {
    BussLegacyMsg   slots[BL_QUEUE_N];
    size_t          head;        /* indeksi i leximit                         */
    size_t          tail;        /* indeksi i shkrimit                        */
    size_t          count;       /* mesazhe aktive                            */
    pthread_mutex_t mutex;
    int             initialized;
} BussLegacyState;

static BussLegacyState g_bl = {
    .head = 0, .tail = 0, .count = 0, .initialized = 0
};

/* ── §1  INIT ────────────────────────────────────────────────────────────── */

int buss_legacy_init(void) {
    if (g_bl.initialized) {
        return BL_OK;
    }
    if (pthread_mutex_init(&g_bl.mutex, NULL) != 0) {
        return BL_ERR_NOT_INIT;
    }
    g_bl.head        = 0;
    g_bl.tail        = 0;
    g_bl.count       = 0;
    g_bl.initialized = 1;
    return BL_OK;
}

/* ── §2  SEND — kopjon të verbër ─────────────────────────────────────────── */

int buss_legacy_send(const BussLegacyMsg *msg) {
    if (msg == NULL) {
        return BL_ERR_PARAM;
    }
    if (!g_bl.initialized) {
        return BL_ERR_NOT_INIT;
    }

    pthread_mutex_lock(&g_bl.mutex);

    if (g_bl.count >= BL_QUEUE_N) {
        pthread_mutex_unlock(&g_bl.mutex);
        return BL_ERR_FULL;
    }

    /*
     * KOPJIM I VERBËR — i gjithë struct-i, përfshirë flags, bit-për-bit.
     * Bus-i nuk e di se brenda flags fshihet vula 500. As nuk e shikon.
     */
    memcpy(&g_bl.slots[g_bl.tail], msg, sizeof(BussLegacyMsg));
    g_bl.tail = (g_bl.tail + 1) & BL_QUEUE_MASK;
    g_bl.count += 1;

    pthread_mutex_unlock(&g_bl.mutex);
    return BL_OK;
}

/* ── §3  RECV — FIFO ─────────────────────────────────────────────────────── */

int buss_legacy_recv(BussLegacyMsg *msg_out) {
    if (msg_out == NULL) {
        return BL_ERR_PARAM;
    }
    if (!g_bl.initialized) {
        return BL_ERR_NOT_INIT;
    }

    pthread_mutex_lock(&g_bl.mutex);

    if (g_bl.count == 0) {
        pthread_mutex_unlock(&g_bl.mutex);
        return BL_ERR_EMPTY;
    }

    memcpy(msg_out, &g_bl.slots[g_bl.head], sizeof(BussLegacyMsg));
    g_bl.head = (g_bl.head + 1) & BL_QUEUE_MASK;
    g_bl.count -= 1;

    pthread_mutex_unlock(&g_bl.mutex);
    return BL_OK;
}

/* ── §4  PENDING / FLUSH ─────────────────────────────────────────────────── */

size_t buss_legacy_pending(void) {
    size_t n;
    if (!g_bl.initialized) {
        return 0;
    }
    pthread_mutex_lock(&g_bl.mutex);
    n = g_bl.count;
    pthread_mutex_unlock(&g_bl.mutex);
    return n;
}

void buss_legacy_flush(void) {
    if (!g_bl.initialized) {
        return;
    }
    pthread_mutex_lock(&g_bl.mutex);
    g_bl.head  = 0;
    g_bl.tail  = 0;
    g_bl.count = 0;
    pthread_mutex_unlock(&g_bl.mutex);
}
