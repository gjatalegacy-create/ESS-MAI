/* hw_colddown.h - ESS-MAI Hardware ColdDown Layer
 * GJATA LEGACY - Quantum Platform
 *
 * Commands hardware cooling components:
 *   - Fan via /sys/class/hwmon/hwmon0/pwm1
 *   - Cooling device via /sys/class/thermal/cooling_device0/cur_state
 *   - CPU throttle via /sys/devices/system/cpu/.../scaling_max_freq
 *
 * Called ONLY from Cold_Down Rust module.
 * Activated ONLY after signal from Cold_Heat_Manager.
 */

#ifndef HW_COLDDOWN_H
#define HW_COLDDOWN_H

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Return codes */
#define HW_CD_OK              0
#define HW_CD_ERR_NULL       -1
#define HW_CD_ERR_WRITE      -2
#define HW_CD_ERR_RANGE      -3
#define HW_CD_ERR_NO_ACCESS  -4

/* Sysfs paths */
#define CD_FAN_PWM_PATH       "/sys/class/hwmon/hwmon0/pwm1"
#define CD_FAN_PWM_EN_PATH    "/sys/class/hwmon/hwmon0/pwm1_enable"
#define CD_COOLING_DEV_PATH   "/sys/class/thermal/cooling_device0/cur_state"
#define CD_COOLING_MAX_PATH   "/sys/class/thermal/cooling_device0/max_state"
#define CD_CPU_FREQ_MAX_PATH  "/sys/devices/system/cpu/cpu0/cpufreq/scaling_max_freq"
#define CD_CPU_FREQ_AVAIL     "/sys/devices/system/cpu/cpu0/cpufreq/scaling_available_frequencies"

/* Fan speed constants */
#define CD_FAN_PWM_MIN        0u
#define CD_FAN_PWM_MAX        255u
#define CD_FAN_NORMAL_PCT     40u
#define CD_FAN_WARN_PCT       70u
#define CD_FAN_EMERGENCY_PCT  100u

/*
 * ColdDownAction - action requested by Cold_Heat_Manager
 */
typedef enum {
    CD_ACTION_NORMAL    = 0,
    CD_ACTION_WARN      = 1,
    CD_ACTION_COOL      = 2,
    CD_ACTION_THROTTLE  = 3,
    CD_ACTION_EMERGENCY = 4
} ColdDownAction;

/*
 * ColdDownResult - result after execution
 */
typedef struct {
    int     rc;
    bool    fan_set;
    bool    cooling_set;
    bool    throttle_set;
    uint8_t fan_pct_applied;
} ColdDownResult;

/* Public API */
int  hw_colddown_execute(ColdDownAction action, ColdDownResult *result);
int  hw_colddown_fan_set_pct(uint8_t speed_pct);
int  hw_colddown_cooling_set_level(uint32_t level);
int  hw_colddown_throttle_cpu(void);
int  hw_colddown_restore_normal(void);
int  hw_colddown_sysfs_available(void);
void hw_colddown_debug_print(const ColdDownResult *result);

#ifdef __cplusplus
}
#endif

#endif /* HW_COLDDOWN_H */
