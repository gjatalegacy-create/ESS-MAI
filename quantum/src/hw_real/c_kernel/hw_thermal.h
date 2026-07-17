/* hw_thermal.h - ESS-MAI Hardware Thermal Layer
 * GJATA LEGACY - Quantum Platform
 *
 * Two modes:
 *   1. IoT/sysfs sensor -> direct temperature read
 *   2. Math formula     -> compute from load (no sensor)
 *
 * Formula (no sensor):
 *   P_cpu   = TDP_base_W x (cpu_load / 100.0)
 *   P_mem   = ram_active_gb x MEM_POWER_PER_GB_W
 *   P_total = P_cpu + P_mem
 *   T_est   = T_AMBIENT_C + P_total x R_THERMAL_C_PER_W
 *   T_LIMIT = T_JUNCTION_MAX_C x THERMAL_SAFETY_RATIO
 *   overheat = (T_est >= T_LIMIT)
 */

#ifndef HW_THERMAL_H
#define HW_THERMAL_H

#include "hw_resource.h"
#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Return codes */
#define HW_THERMAL_OK            0
#define HW_THERMAL_ERR_NULL     -1
#define HW_THERMAL_ERR_READ     -2
#define HW_THERMAL_ERR_INVALID  -3

/* Physical constants (typical values - calibrate per device) */
#define THERMAL_TDP_BASE_W          65.0f
#define THERMAL_MEM_POWER_PER_GB_W   3.0f
#define THERMAL_R_C_PER_W            0.5f
#define THERMAL_T_AMBIENT_C         25.0f
#define THERMAL_T_JUNCTION_MAX_C   100.0f
#define THERMAL_SAFETY_RATIO         0.85f
#define THERMAL_WARN_RATIO           0.75f

/* Sysfs paths */
#define THERMAL_SYSFS_ZONE0   "/sys/class/thermal/thermal_zone0/temp"
#define THERMAL_SYSFS_ZONE1   "/sys/class/thermal/thermal_zone1/temp"
#define THERMAL_SYSFS_HWMON0  "/sys/class/hwmon/hwmon0/temp1_input"

/*
 * ThermalConfig - device-specific calibration
 */
typedef struct {
    float tdp_base_w;
    float mem_power_per_gb_w;
    float r_thermal_c_per_w;
    float t_ambient_c;
    float t_junction_max_c;
    float safety_ratio;
} ThermalConfig;

extern const ThermalConfig THERMAL_CONFIG_DEFAULT;

/*
 * ThermalSnapshot - thermal state at one moment
 *
 * Invariants:
 *   t_estimated_c  in (-50, 300)
 *   t_limit_c      > 0
 *   p_total_w      >= 0
 *
 * Layout note: 7 floats (28 bytes) + 3 bools (3 bytes)
 * + 5 bytes padding + ts_ns (8 bytes) = 44 bytes total.
 * The _pad[5] makes this explicit for Rust FFI alignment.
 */
typedef struct {
    float    t_estimated_c;
    float    t_limit_c;
    float    t_warn_c;
    float    t_junction_max_c;
    float    p_cpu_w;
    float    p_mem_w;
    float    p_total_w;
    bool     sensor_used;
    bool     overheat;
    bool     warning;
    uint8_t  _pad[5];
    uint64_t ts_ns;
} ThermalSnapshot;

/* Public API */
int  hw_thermal_read(const HwSnapshot *hw,
                     const ThermalConfig *cfg,
                     ThermalSnapshot *out);
int  hw_thermal_iot_available(void);
int  hw_thermal_validate(const ThermalSnapshot *snap);
void hw_thermal_debug_print(const ThermalSnapshot *snap);

#ifdef __cplusplus
}
#endif

#endif /* HW_THERMAL_H */
