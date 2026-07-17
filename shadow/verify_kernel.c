/* ╔══════════════════════════════════════════════════════════════════════════╗
 * ║  VERIFY_KERNEL.C — VERIFIKIM RUNTIME i kernelit suprem (HAPI 2 / 100%) ║
 * ║  Gjata Legacy™ | Bledar Gjata                                         ║
 * ║                                                                          ║
 * ║  STANDALONE (jo pjesë e cargo build). Ekzekuton kernelin real dhe      ║
 * ║  verifikon sjelljen e vendimit 0/1: vula 500, ngrirja, statistikat,    ║
 * ║  dhe siguria ndaj NULL.                                                 ║
 * ║                                                                          ║
 * ║  Kompilim + ekzekutim:                                                  ║
 * ║    gcc -std=c11 -Wall -Wextra -O3 -Ikernel verify_kernel.c \           ║
 * ║        kernel/shadow_buss.c kernel/buss_legacy.c kernel/shadow_gj_legacy.c \
 * ║        -lpthread -o verify_kernel && ./verify_kernel                    ║
 * ╚══════════════════════════════════════════════════════════════════════════╝ */

#include <stdio.h>
#include <string.h>
#include <stdint.h>
#include "shadow_gj_legacy.h"

static int g_fail = 0;
static int g_pass = 0;

#define CHECK(cond, msg) do {                         \
    if (cond) { g_pass++; printf("    [OK] %s\n", msg); }   \
    else      { g_fail++; printf("    [XX] FAIL: %s\n", msg); } \
} while (0)

static LgcRequest mk_req(uint32_t flags, float ram) {
    LgcRequest r;
    memset(&r, 0, sizeof(r));
    r.flags     = flags;
    r.ram_usage = ram;
    r.ts        = 0;
    return r;
}

int main(void) {
    LgcStats  st;
    LgcResult res;

    printf("\n=== VERIFIKIM RUNTIME — shadow_gj_legacy.c (vendimi suprem 0/1) ===\n\n");

    /* [1] init + stats fillestare */
    printf("[1] lgc_init + gjendje fillestare:\n");
    CHECK(lgc_init() == 0, "lgc_init() == 0");
    lgc_get_stats(&st);
    CHECK(st.total == 0 && st.passed == 0 && st.blocked == 0, "stats = 0 pas init");
    CHECK(st.frozen == 0, "frozen == 0 pas init");

    /* [2] aritmetika e vulës 500 (pa state) */
    printf("\n[2] shadow_lgc_seal_check — (flags & 0xFFFF) ^ 0xA5A5 == 500:\n");
    CHECK(shadow_lgc_seal_check(0xA451) == 1, "0xA451 -> SEALED (1)");
    CHECK(shadow_lgc_seal_check(0x0000) == 0, "0x0000 -> STRIPPED (0)");
    CHECK(shadow_lgc_seal_check(500)    == 0, "500 (pa XOR) -> 0");
    CHECK(shadow_lgc_seal_check(0xFFFF) == 0, "0xFFFF -> 0");

    /* [3] sealed + jo i ngrirë + ram i ulët -> PASS + LIGHT */
    printf("\n[3] lgc_check — sealed, jo i ngrirë, ram i ulët:\n");
    memset(&res, 0, sizeof(res));
    LgcRequest r_seal = mk_req(0xA451, 0.10f);
    CHECK(lgc_check(&r_seal, &res) == 0, "lgc_check() == 0");
    CHECK(res.decision == LGC_DECISION_PASS, "decision == PASS (sealed & live)");
    CHECK(res.state    == LGC_STATE_LIGHT,   "state == LIGHT (ram<=0.5)");

    /* [4] stripped -> BLOCK */
    printf("\n[4] lgc_check — stripped flags:\n");
    LgcRequest r_strip = mk_req(0x0000, 0.10f);
    lgc_check(&r_strip, &res);
    CHECK(res.decision == LGC_DECISION_BLOCK, "decision == BLOCK (stripped)");

    /* [5] ram i lartë -> PRESSURED, por vendimi mbetet PASS (ram s'prek vendimin) */
    printf("\n[5] lgc_check — sealed por ram i lartë:\n");
    LgcRequest r_press = mk_req(0xA451, 0.90f);
    lgc_check(&r_press, &res);
    CHECK(res.state    == LGC_STATE_PRESSURED, "state == PRESSURED (ram>0.5)");
    CHECK(res.decision == LGC_DECISION_PASS,   "decision ende PASS (ram s'prek vendimin)");

    /* [6] freeze -> bllokon edhe sealed */
    printf("\n[6] lgc_freeze — ngrirja sovrane bllokon gjithçka:\n");
    CHECK(lgc_freeze(1) == 0, "lgc_freeze(1) == 0");
    lgc_get_stats(&st);
    CHECK(st.frozen == 1, "frozen == 1");
    lgc_check(&r_seal, &res);
    CHECK(res.decision == LGC_DECISION_BLOCK, "sealed -> BLOCK kur i ngrirë");

    /* [7] unfreeze -> PASS rikthehet */
    printf("\n[7] lgc_unfreeze — rikthim:\n");
    CHECK(lgc_unfreeze() == 0, "lgc_unfreeze() == 0");
    lgc_get_stats(&st);
    CHECK(st.frozen == 0, "frozen == 0");
    lgc_check(&r_seal, &res);
    CHECK(res.decision == LGC_DECISION_PASS, "sealed -> PASS pas unfreeze");

    /* [8] freeze(stage) ruan stage; freeze(0) -> default 1 */
    printf("\n[8] lgc_freeze(stage):\n");
    lgc_freeze(7); lgc_get_stats(&st);
    CHECK(st.frozen == 7, "freeze(7) -> frozen == 7");
    lgc_freeze(0); lgc_get_stats(&st);
    CHECK(st.frozen == 1, "freeze(0) -> frozen == 1 (default)");
    lgc_unfreeze();

    /* [9] kontabiliteti total/passed/blocked */
    printf("\n[9] kontabiliteti i statistikave:\n");
    lgc_init(); /* reset */
    for (int i = 0; i < 5; i++) { LgcRequest a = mk_req(0xA451, 0.1f); lgc_check(&a, &res); }
    for (int i = 0; i < 3; i++) { LgcRequest a = mk_req(0x0000, 0.1f); lgc_check(&a, &res); }
    lgc_get_stats(&st);
    CHECK(st.total   == 8, "total == 8");
    CHECK(st.passed  == 5, "passed == 5");
    CHECK(st.blocked == 3, "blocked == 3");
    CHECK(st.passed + st.blocked == st.total, "passed + blocked == total");

    /* [10] siguria ndaj NULL */
    printf("\n[10] siguria ndaj NULL:\n");
    CHECK(lgc_check(NULL, &res)     == -1, "lgc_check(NULL, res) == -1");
    CHECK(lgc_check(&r_seal, NULL)  == -1, "lgc_check(req, NULL) == -1");

    printf("\n=== REZULTATI: %d kaluan, %d deshtuan ===\n", g_pass, g_fail);
    printf("%s\n\n", g_fail == 0 ? "SOVEREIGN_KERNEL_RUNTIME = OK" : "KERNEL_RUNTIME = FAIL");
    return g_fail == 0 ? 0 : 1;
}
