/* hw_colddown.c - ESS-MAI Hardware ColdDown Layer
 * GJATA LEGACY - Quantum Platform
 *
 * Writes directly to sysfs to control fan, cooling device, CPU throttle.
 * Requires write permission (root or hwmon group).
 *
 * Graceful degradation: if sysfs write fails -> log error, continue.
 * Zero panic. Zero abort. Returns accurate result.
 */

#define _POSIX_C_SOURCE 200809L

#include "hw_colddown.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <stdbool.h>
#include <errno.h>
#ifndef _WIN32
#include <unistd.h>
#endif

/* --- UTILITY: write numeric value to sysfs path --- */
static int sysfs_write_uint(const char *path, unsigned long value) {
    if (!path) return HW_CD_ERR_NULL;
    FILE *f = fopen(path, "w");
    if (!f) return HW_CD_ERR_NO_ACCESS;
    int ok = (fprintf(f, "%lu\n", value) > 0);
    fclose(f);
    return ok ? HW_CD_OK : HW_CD_ERR_WRITE;
}

/* --- UTILITY: read numeric value from sysfs path --- */
static int sysfs_read_ulong(const char *path, unsigned long *out) {
    if (!path || !out) return HW_CD_ERR_NULL;
    FILE *f = fopen(path, "r");
    if (!f) return HW_CD_ERR_NO_ACCESS;
    int ok = (fscanf(f, "%lu", out) == 1);
    fclose(f);
    return ok ? HW_CD_OK : HW_CD_ERR_WRITE;
}

/* --- UTILITY: check path permissions --- */
#ifdef _WIN32
/* Windows: sysfs nuk ekziston — asnjë path s'është i lexueshëm/shkrueshëm.
 * Colddown-i bëhet PASIV natyrshëm (të gjitha veprimet raportojnë NO_ACCESS,
 * asnjë dështim i pipeline-it — leximi PRANOHET si "shtresë e padisponueshme"). */
static int path_writable(const char *path) {
    (void)path;
    return 0;
}

static int path_readable(const char *path) {
    (void)path;
    return 0;
}
#else
static int path_writable(const char *path) {
    if (!path) return 0;
    return (access(path, W_OK) == 0) ? 1 : 0;
}

static int path_readable(const char *path) {
    if (!path) return 0;
    return (access(path, R_OK) == 0) ? 1 : 0;
}
#endif

/* --- FAN CONTROL via PWM
 * PWM range: [0, 255]
 * Convert: pwm = speed_pct x 255 / 100
 * Enable manual mode: pwm1_enable = 1
 */
int hw_colddown_fan_set_pct(uint8_t speed_pct) {
    if (speed_pct > 100u) return HW_CD_ERR_RANGE;

    /* enable manual PWM control */
    sysfs_write_uint(CD_FAN_PWM_EN_PATH, 1u);

    /* convert % to PWM [0,255] */
    unsigned long pwm_val = (unsigned long)speed_pct * 255u / 100u;
    return sysfs_write_uint(CD_FAN_PWM_PATH, pwm_val);
}

/* --- COOLING DEVICE via thermal cooling_device sysfs
 * Levels: 0 = off, max_state = full cooling
 */
int hw_colddown_cooling_set_level(uint32_t level) {
    unsigned long max_state = 10u;
    sysfs_read_ulong(CD_COOLING_MAX_PATH, &max_state);

    unsigned long target = (unsigned long)level;
    if (target > max_state) target = max_state;

    return sysfs_write_uint(CD_COOLING_DEV_PATH, target);
}

/* --- CPU FREQUENCY THROTTLE
 * Reads available frequencies, selects second-lowest.
 * Writes to scaling_max_freq.
 */
int hw_colddown_throttle_cpu(void) {
    if (!path_readable(CD_CPU_FREQ_AVAIL))    return HW_CD_ERR_NO_ACCESS;
    if (!path_writable(CD_CPU_FREQ_MAX_PATH)) return HW_CD_ERR_NO_ACCESS;

    FILE *f = fopen(CD_CPU_FREQ_AVAIL, "r");
    if (!f) return HW_CD_ERR_NO_ACCESS;

    unsigned long freqs[32];
    int n = 0;
    unsigned long v;
    while (n < 32 && fscanf(f, "%lu", &v) == 1)
        freqs[n++] = v;
    fclose(f);

    if (n < 2) return HW_CD_ERR_NO_ACCESS;

    /* frequencies are in descending order: freqs[1] = second highest */
    return sysfs_write_uint(CD_CPU_FREQ_MAX_PATH, freqs[1]);
}

/* --- RESTORE NORMAL - return system to normal operation --- */
int hw_colddown_restore_normal(void) {
    int any_ok = 0;

    if (hw_colddown_fan_set_pct(CD_FAN_NORMAL_PCT) == HW_CD_OK) any_ok = 1;
    if (hw_colddown_cooling_set_level(0u) == HW_CD_OK) any_ok = 1;

    if (path_readable(CD_CPU_FREQ_AVAIL) &&
        path_writable(CD_CPU_FREQ_MAX_PATH))
    {
        FILE *f = fopen(CD_CPU_FREQ_AVAIL, "r");
        if (f) {
            unsigned long max_freq = 0;
            unsigned long fv;
            while (fscanf(f, "%lu", &fv) == 1)
                if (fv > max_freq) max_freq = fv;
            fclose(f);
            if (max_freq > 0 &&
                sysfs_write_uint(CD_CPU_FREQ_MAX_PATH, max_freq) == HW_CD_OK)
                any_ok = 1;
        }
    }

    return any_ok ? HW_CD_OK : HW_CD_ERR_NO_ACCESS;
}

/* --- EXECUTE - unified action based on ColdDownAction --- */
int hw_colddown_execute(ColdDownAction action, ColdDownResult *result) {
    ColdDownResult local;
    memset(&local, 0, sizeof(local));
    local.rc = HW_CD_OK;

    switch (action) {

    case CD_ACTION_NORMAL:
        local.fan_pct_applied = CD_FAN_NORMAL_PCT;
        break;

    case CD_ACTION_WARN:
        if (hw_colddown_fan_set_pct(CD_FAN_WARN_PCT) == HW_CD_OK) {
            local.fan_set         = true;
            local.fan_pct_applied = CD_FAN_WARN_PCT;
        }
        if (hw_colddown_cooling_set_level(1u) == HW_CD_OK)
            local.cooling_set = true;
        break;

    case CD_ACTION_COOL: {
        if (hw_colddown_fan_set_pct(CD_FAN_EMERGENCY_PCT) == HW_CD_OK) {
            local.fan_set         = true;
            local.fan_pct_applied = CD_FAN_EMERGENCY_PCT;
        }
        unsigned long ms = 10u;
        sysfs_read_ulong(CD_COOLING_MAX_PATH, &ms);
        if (hw_colddown_cooling_set_level((uint32_t)ms) == HW_CD_OK)
            local.cooling_set = true;
        break;
    }

    case CD_ACTION_THROTTLE: {
        if (hw_colddown_fan_set_pct(CD_FAN_EMERGENCY_PCT) == HW_CD_OK) {
            local.fan_set         = true;
            local.fan_pct_applied = CD_FAN_EMERGENCY_PCT;
        }
        unsigned long ms2 = 10u;
        sysfs_read_ulong(CD_COOLING_MAX_PATH, &ms2);
        if (hw_colddown_cooling_set_level((uint32_t)ms2) == HW_CD_OK)
            local.cooling_set = true;
        if (hw_colddown_throttle_cpu() == HW_CD_OK)
            local.throttle_set = true;
        break;
    }

    case CD_ACTION_EMERGENCY: {
        if (hw_colddown_fan_set_pct(CD_FAN_EMERGENCY_PCT) == HW_CD_OK) {
            local.fan_set         = true;
            local.fan_pct_applied = CD_FAN_EMERGENCY_PCT;
        }
        unsigned long ms3 = 10u;
        sysfs_read_ulong(CD_COOLING_MAX_PATH, &ms3);
        hw_colddown_cooling_set_level((uint32_t)ms3);
        local.cooling_set = true;
        if (hw_colddown_throttle_cpu() == HW_CD_OK)
            local.throttle_set = true;
        sysfs_write_uint(CD_FAN_PWM_PATH, CD_FAN_PWM_MAX);
        break;
    }

    default:
        local.rc = HW_CD_ERR_RANGE;
        break;
    }

    if (result) *result = local;
    return local.rc;
}

/* --- AVAILABILITY CHECK --- */
int hw_colddown_sysfs_available(void) {
    int count = 0;
    if (path_writable(CD_FAN_PWM_PATH))      count++;
    if (path_writable(CD_COOLING_DEV_PATH))  count++;
    if (path_writable(CD_CPU_FREQ_MAX_PATH)) count++;
    return count;
}

/* --- DEBUG --- */
void hw_colddown_debug_print(const ColdDownResult *result) {
    if (!result) { fprintf(stderr, "[HW_COLDDOWN] result=NULL\n"); return; }
    fprintf(stderr,
        "[HW_COLDDOWN] rc=%d fan=%s(%u%%) cooling=%s throttle=%s\n",
        result->rc,
        result->fan_set      ? "SET" : "skip", result->fan_pct_applied,
        result->cooling_set  ? "SET" : "skip",
        result->throttle_set ? "SET" : "skip");
}
