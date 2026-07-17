/* hw_thermal.c - ESS-MAI Hardware Thermal Layer
 * GJATA LEGACY - Quantum Platform
 *
 * Thermal formula (no IoT sensor):
 *   P_cpu   = TDP_base_W x (cpu_load / 100.0)
 *   P_mem   = ram_active_gb x MEM_POWER_PER_GB_W
 *   P_total = P_cpu + P_mem
 *   T_est   = T_ambient + P_total x R_thermal
 *   T_limit = T_junction_max x safety_ratio (0.85)
 *
 * WINDOWS: sysfs nuk ekziston → fopen dështon natyrshëm → sensor_used=false
 * dhe FORMULA MATEMATIKE merr përsipër automatikisht (leximi PRANOHET gjithmonë,
 * kurrë s'dështon për mungesë sensori — kjo është sjellja e pranuar sovrane).
 */

#ifndef _WIN32
#define _POSIX_C_SOURCE 200809L
#endif

#include "hw_thermal.h"
#include "hw_resource.h"

#include <stdio.h>
#include <string.h>
#include <stdint.h>
#include <stdbool.h>
#include <time.h>

#ifdef _WIN32
#include <windows.h>
#else
#include <unistd.h>
#endif

/* Config default */
const ThermalConfig THERMAL_CONFIG_DEFAULT = {
    THERMAL_TDP_BASE_W,
    THERMAL_MEM_POWER_PER_GB_W,
    THERMAL_R_C_PER_W,
    THERMAL_T_AMBIENT_C,
    THERMAL_T_JUNCTION_MAX_C,
    THERMAL_SAFETY_RATIO
};

#ifdef _WIN32
static uint64_t thermal_ts_ns(void) {
    LARGE_INTEGER freq, now;
    if (!QueryPerformanceFrequency(&freq) || freq.QuadPart <= 0) return 0ULL;
    if (!QueryPerformanceCounter(&now))                          return 0ULL;
    uint64_t sec = (uint64_t)now.QuadPart / (uint64_t)freq.QuadPart;
    uint64_t rem = (uint64_t)now.QuadPart % (uint64_t)freq.QuadPart;
    return sec * 1000000000ULL + rem * 1000000000ULL / (uint64_t)freq.QuadPart;
}
#else
static uint64_t thermal_ts_ns(void) {
    struct timespec ts;
    if (clock_gettime(CLOCK_MONOTONIC, &ts) != 0) return 0ULL;
    return (uint64_t)ts.tv_sec * 1000000000ULL + (uint64_t)ts.tv_nsec;
}
#endif

/* --- SENSOR SYSFS ---
 * /sys/class/thermal/thermal_zoneN/temp -> value in milli-degC
 * e.g. 45000 -> 45.0 degC
 */
static int try_read_sysfs_temp(const char *path, float *out_c) {
    if (!path || !out_c) return 0;

    FILE *f = fopen(path, "r");
    if (!f) return 0;

    int raw_milli = 0;
    int ok = (fscanf(f, "%d", &raw_milli) == 1);
    fclose(f);

    if (!ok) return 0;

    /* convert milli-degC to degC */
    *out_c = (float)raw_milli / 1000.0f;

    /* sanity check: [-20, 200] degC */
    if (*out_c < -20.0f || *out_c > 200.0f) return 0;
    return 1;
}

int hw_thermal_iot_available(void) {
    /* Probë reale e sysfs: na intereson VETËM a lexohet një sensor real
     * (vlera hidhet te probe_c). Zero simulim — kjo prek hardware-in fizik. */
    float probe_c;
    if (try_read_sysfs_temp(THERMAL_SYSFS_ZONE0,  &probe_c)) return 1;
    if (try_read_sysfs_temp(THERMAL_SYSFS_ZONE1,  &probe_c)) return 1;
    if (try_read_sysfs_temp(THERMAL_SYSFS_HWMON0, &probe_c)) return 1;
    return 0;
}

static int read_sensor_temp(float *out_c) {
    if (!out_c) return 0;
    if (try_read_sysfs_temp(THERMAL_SYSFS_ZONE0,  out_c)) return 1;
    if (try_read_sysfs_temp(THERMAL_SYSFS_ZONE1,  out_c)) return 1;
    if (try_read_sysfs_temp(THERMAL_SYSFS_HWMON0, out_c)) return 1;
    return 0;
}

/* --- MATH FORMULA ---
 * P_cpu = tdp_base x (cpu_load / 100)
 * P_mem = ram_active_gb x mem_power_per_gb
 * T_est = T_ambient + P_total x R_thermal
 */
static void compute_thermal_math(
    const HwSnapshot    *hw,
    const ThermalConfig *cfg,
    float *p_cpu_w,
    float *p_mem_w,
    float *p_total_w,
    float *t_est_c)
{
    *p_cpu_w = cfg->tdp_base_w * (hw->cpu_load_pct / 100.0f);

    uint64_t ram_used = (hw->ram_total_bytes > hw->ram_free_bytes)
                      ? (hw->ram_total_bytes - hw->ram_free_bytes)
                      : 0ULL;
    float ram_gb = (float)ram_used / (1024.0f * 1024.0f * 1024.0f);

    *p_mem_w   = ram_gb * cfg->mem_power_per_gb_w;
    *p_total_w = *p_cpu_w + *p_mem_w;
    *t_est_c   = cfg->t_ambient_c + (*p_total_w) * cfg->r_thermal_c_per_w;
}

/* --- PUBLIC API --- */

int hw_thermal_read(const HwSnapshot    *hw,
                    const ThermalConfig *cfg,
                    ThermalSnapshot     *out)
{
    if (!hw || !out)              return HW_THERMAL_ERR_NULL;
    if (!hw_resource_validate(hw)) return HW_THERMAL_ERR_INVALID;

    const ThermalConfig *c = (cfg != NULL) ? cfg : &THERMAL_CONFIG_DEFAULT;

    memset(out, 0, sizeof(ThermalSnapshot));

    out->t_junction_max_c = c->t_junction_max_c;
    out->t_limit_c        = c->t_junction_max_c * c->safety_ratio;
    out->t_warn_c         = c->t_junction_max_c * THERMAL_WARN_RATIO;
    out->ts_ns            = thermal_ts_ns();

    /* always compute math (for P values) */
    compute_thermal_math(hw, c,
                         &out->p_cpu_w,
                         &out->p_mem_w,
                         &out->p_total_w,
                         &out->t_estimated_c);

    /* try IoT sensor - overwrite t_estimated if available */
    float sensor_t = 0.0f;
    if (read_sensor_temp(&sensor_t)) {
        out->t_estimated_c = sensor_t;
        out->sensor_used   = true;
    } else {
        out->sensor_used   = false;
    }

    out->overheat = (out->t_estimated_c >= out->t_limit_c);
    out->warning  = (!out->overheat &&
                     out->t_estimated_c >= out->t_warn_c);

    if (!hw_thermal_validate(out)) return HW_THERMAL_ERR_INVALID;
    return HW_THERMAL_OK;
}

int hw_thermal_validate(const ThermalSnapshot *snap) {
    if (!snap)                           return 0;
    if (snap->t_estimated_c  < -50.0f)  return 0;
    if (snap->t_estimated_c  > 300.0f)  return 0;
    if (snap->t_limit_c      <= 0.0f)   return 0;
    if (snap->t_junction_max_c <= 0.0f) return 0;
    if (snap->p_total_w       < 0.0f)   return 0;
    if (snap->p_cpu_w         < 0.0f)   return 0;
    if (snap->p_mem_w         < 0.0f)   return 0;
    return 1;
}

void hw_thermal_debug_print(const ThermalSnapshot *snap) {
    if (!snap) { fprintf(stderr, "[HW_THERMAL] snap=NULL\n"); return; }
    fprintf(stderr,
        "[HW_THERMAL] T=%.1fC (limit=%.1f warn=%.1f max=%.1f)"
        " P_cpu=%.1fW P_mem=%.1fW P_tot=%.1fW"
        " sensor=%s overheat=%s warn=%s\n",
        snap->t_estimated_c, snap->t_limit_c,
        snap->t_warn_c, snap->t_junction_max_c,
        snap->p_cpu_w, snap->p_mem_w, snap->p_total_w,
        snap->sensor_used ? "YES" : "MATH",
        snap->overheat    ? "YES" : "no",
        snap->warning     ? "YES" : "no");
}
