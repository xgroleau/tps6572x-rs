use modular_bitfield::prelude::*;

trait Register {
    const ADDRESS: RegisterAddress;
}
trait WritableRegister: Register {}

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum RegisterAddress {
    /// CHGSTATUS
    BatteryChargerStatus = 0x01,

    /// CHGCONFIGG0
    ChargerConfigControl0 = 0x02,

    /// CHGCONFIGG1
    BatteryChargerConfigControl1 = 0x03,

    /// CHGCONFIGG2
    BatteryChargerConfigControl2 = 0x04,

    /// CHGCONFIGG3
    BatteryChargerConfigControl3 = 0x05,

    /// CHGSTATE
    ChargerStatus = 0x06,

    /// DEFDCDC1
    DCDCSettingControl = 0x07,

    /// LDOCTRL
    LDOSettingControl = 0x08,

    /// CONTROL0
    Control0 = 0x09,

    /// CONTROL1
    Control1 = 0x0A,

    /// GPIOSCC
    GPIOSCC = 0x0B,

    /// GPIODIR
    GPIOConfigControl = 0x0C,

    /// IRMARSK0
    InterruptMask0 = 0xD0,

    /// IRMASK1
    InterruptMask1 = 0xE0,

    /// IRMASK2
    InterruptMask2 = 0xF0,

    /// IR0
    InterruptReporting0 = 0x10,

    /// IR1
    InterruptReporting1 = 0x11,

    /// IR2
    InterruptReporting2 = 0x12,
}

/// CHGSTATUS register
#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct ChargerStatus {
    /// Bit 0 skipped
    #[skip]
    pub __: B1,

    /// CH_THLOOP
    #[skip(setters)]
    pub thermal_loop_active: bool,

    /// CH_PGOOD
    #[skip(setters)]
    pub power_source_ok: bool,

    /// CH_ACTIVE
    #[skip(setters)]
    pub charger_active: bool,

    /// Bit 4 skipped
    #[skip]
    pub __: B1,

    /// OVP
    #[skip(setters)]
    pub over_voltage_protection: bool,

    /// TS_COLD
    #[skip(setters)]
    pub temp_low: bool,

    /// TS_HOT
    #[skip(setters)]
    pub temp_high: bool,
}
impl Register for ChargerStatus {
    const ADDRESS: RegisterAddress = RegisterAddress::ChargerStatus;
}

/// Represents the possible output voltage for the `[ChargerConfig0]`
#[derive(Copy, Clone, PartialEq, Debug, BitfieldSpecifier)]
#[bits = 2]
pub enum SysOutputVoltage {
    // Output voltage of ther power path at SYS pin
    Vsys = 0b00,

    // 4.4V
    V4_4 = 0b01,

    // 5V
    V5 = 0b10,

    // 5.5V
    V5_5 = 0b11,
}

/// Represents the possible input current for the `[ChargerConfig0]`
#[derive(Copy, Clone, PartialEq, Debug, BitfieldSpecifier)]
#[bits = 2]
pub enum AcInputCurrent {
    /// 100mA, with DDPM
    MA100ddpm = 0b00,

    /// 500mA, with DDPM
    MA500ddpm = 0b01,

    /// 500mA, without DDPM
    MA500 = 0b10,

    /// Usb suspend mode, standby
    UsbSuspend = 0b11,
}

/// CHGCONFIG0 register
#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct ChargerConfig0 {
    /// CH_EN
    pub charger_enabled: bool,

    /// TERM_EN
    pub termination_enabled: bool,

    /// DYN_TMR
    pub dynamic_timer: bool,

    /// TH_LOOP
    pub thermal_loop: bool,

    /// Ac input
    pub ac_input_current: AcInputCurrent,

    /// VSYS0-1
    pub output_voltage: SysOutputVoltage,
}

/// Termination current scaling factor
#[derive(Copy, Clone, PartialEq, Debug, BitfieldSpecifier)]
#[bits = 2]
pub enum TerminationCurrentScalingFactor {
    P5 = 0b00,
    P10 = 0b01,
    P15 = 0b10,
    P20 = 0b11,
}

/// Charge current factor
#[derive(Copy, Clone, PartialEq, Debug, BitfieldSpecifier)]
#[bits = 2]
pub enum ChargeCurrentFactor {
    P25 = 0b00,
    P50 = 0b01,
    P75 = 0b10,
    P100 = 0b11,
}

/// Pre charge current factor
#[derive(Copy, Clone, PartialEq, Debug, BitfieldSpecifier)]
#[bits = 2]
pub enum PreChargeCurrentFactor {
    P5 = 0b00,
    P10 = 0b01,
    P15 = 0b10,
    P20 = 0b11,
}

/// CHGCONFIG1 register
#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct ChargerConfig1 {
    /// Skip B0 and B1
    #[skip]
    __: B2,

    /// I_TERM0-1
    pub termination_current_factor: TerminationCurrentScalingFactor,

    /// ICH_SCL0-1
    pub charge_current_factor: ChargeCurrentFactor,

    /// I_PRE0-1
    pub pre_charge_current_factor: PreChargeCurrentFactor,
}

/// PowerPath threshold
#[derive(Copy, Clone, PartialEq, Debug, BitfieldSpecifier)]
#[bits = 1]
pub enum PowerBatThreshold {
    VBAT100mV = 0b0,
    V4_3 = 0b1,
}

/// NTC sensor resistance
#[derive(Copy, Clone, PartialEq, Debug, BitfieldSpecifier)]
#[bits = 1]
pub enum NTC {
    K100 = 0b0,
    K10 = 0b1,
}

/// Pre-charge timer value
#[derive(Copy, Clone, PartialEq, Debug, BitfieldSpecifier)]
#[bits = 1]
pub enum PreChargeTimer {
    Min30 = 0b0,
    Min60 = 0b1,
}

/// Charge safety timer
#[derive(Copy, Clone, PartialEq, Debug, BitfieldSpecifier)]
#[bits = 2]
pub enum SafetyChargeTimer {
    Hour4 = 0b00,
    Hour5 = 0b01,
    Hour6 = 0b10,
    Hour8 = 0b11,
}

/// CHGCONFIG2 register
#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct ChargerConfig2 {
    /// Skip B0
    #[skip]
    __: B1,

    /// VBAT_COMP_EN
    pub batt_voltage_comparator_enabled: bool,

    /// V_DDPM
    pub dynamic_power_path_treshold: PowerBatThreshold,

    /// NTC
    pub sensor_resistance: NTC,

    /// Skip B4
    #[skip]
    __: B1,

    /// PRE_TMR
    pub pre_charge_timer: PreChargeTimer,

    /// SFTY_TMR0-1
    pub safety_charge_timer: SafetyChargeTimer,
}

/// Battery voltage comparator threshold
#[derive(Copy, Clone, PartialEq, Debug, BitfieldSpecifier)]
#[bits = 2]
pub enum BatteryVoltageTreshold {
    V2_2 = 0b00,
    V2_3 = 0b01,
    V2_4 = 0b10,
    V2_5 = 0b11,
}

/// Battery temperature shift
#[derive(Copy, Clone, PartialEq, Debug, BitfieldSpecifier)]
#[bits = 2]
pub enum BatteryTemperatureShift {
    C0_45 = 0b00,
    C5_50 = 0b01,
    C10_55 = 0b10,
    C15_60 = 0b11,
}

/// Charge voltage selection
#[derive(Copy, Clone, PartialEq, Debug, BitfieldSpecifier)]
#[bits = 3]
pub enum ChargeVoltage {
    V4_150 = 0b000,
    V4_175 = 0b001,
    V4_200 = 0b010,
    V4_225 = 0b011,
    V4_250 = 0b100,
    V4_275 = 0b101,
    V4_300 = 0b110,
    V4_325 = 0b111,
}

/// CHGCONFIG3 register
#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct ChargerConfig3 {
    /// VBAT_COMP
    #[skip(setters)]
    pub vbatt_comparator: bool,

    /// VBAT0-1
    pub vbatt_treshold: BatteryVoltageTreshold,

    /// TMP_SHIFT
    pub batt_temperature_shift: BatteryTemperatureShift,

    /// CH_VLTG0-2
    pub charge_voltage: ChargeVoltage,
}

/// CHGSTATE register
#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct ChargerState {
    /// CH_SUSP
    #[skip(setters)]
    pub suspended: bool,

    /// CH_FAULT
    #[skip(setters)]
    pub fault: bool,

    /// CH_LDO
    #[skip(setters)]
    pub ldo: bool,

    /// CH_CC_CV
    #[skip(setters)]
    pub constant_current: bool,

    /// CH_PRECH
    #[skip(setters)]
    pub precharge: bool,

    /// CH_IDLE
    #[skip(setters)]
    pub idle: bool,

    /// CH_RESET
    #[skip(setters)]
    pub reset: bool,

    /// CH_SLEEP
    #[skip(setters)]
    pub sleep: bool,
}

/// Charge voltage selection
#[derive(Copy, Clone, PartialEq, Debug, BitfieldSpecifier)]
#[bits = 6]
pub enum OutputVoltage {
    V0_800 = 0,
    V0_825 = 1,
    V0_850 = 2,
    V0_875 = 3,
    V0_900 = 4,
    V0_925 = 5,
    V0_950 = 6,
    V0_975 = 7,
    V1_000 = 8,
    V1_025 = 9,
    V1_050 = 10,
    V1_075 = 11,
    V1_100 = 12,
    V1_125 = 13,
    V1_150 = 14,
    V1_175 = 15,
    V1_200 = 16,
    V1_225 = 17,
    V1_250 = 18,
    V1_275 = 19,
    V1_300 = 20,
    V1_325 = 21,
    V1_350 = 22,
    V1_375 = 23,
    V1_400 = 24,
    V1_425 = 25,
    V1_450 = 26,
    V1_475 = 27,
    V1_500 = 28,
    V1_525 = 29,
    V1_550 = 30,
    V1_575 = 31,
    V1_600 = 32,
    V1_650 = 33,
    V1_700 = 34,
    V1_750 = 35,
    V1_800 = 36,
    V1_850 = 37,
    V1_900 = 38,
    V1_950 = 39,
    V2_000 = 40,
    V2_050 = 41,
    V2_100 = 42,
    V2_150 = 43,
    V2_200 = 44,
    V2_250 = 45,
    V2_300 = 46,
    V2_350 = 47,
    V2_400 = 48,
    V2_450 = 49,
    V2_500 = 50,
    V2_550 = 51,
    V2_600 = 52,
    V2_650 = 53,
    V2_700 = 54,
    V2_750 = 55,
    V2_800 = 56,
    V2_850 = 57,
    V2_900 = 58,
    V2_950 = 59,
    V3_000 = 60,
    V3_100 = 61,
    V3_200 = 62,
    V3_300 = 63,
}

/// DEFDCDC1
#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct DCDCSetting {
    /// DCDC0_5
    pub output_voltage: OutputVoltage,

    /// DCDC_DISCH
    pub discharged_when_disabled: bool,

    /// HOLD_DCDC1
    pub hold: bool,
}

/// LDO_CTLG
#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct LDOControl {
    /// LDO1_0-5
    pub output_voltage: OutputVoltage,

    /// LDO1_DISCH
    pub discharged_when_disabled: bool,

    /// HOLD_LDO1
    pub hold: bool,
}

/// CONTROL0
#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Control0 {
    /// B0-4 not used
    #[skip]
    pub __: B5,

    /// PGOODZ_LDO1
    #[skip(setters)]
    pub good_ldo_range: bool,

    /// PGOODZ_DCDC1
    #[skip(setters)]
    pub good_dcdc_range: bool,

    /// F_PWM
    pub forced_pwm: bool,
}

/// Reset delay
#[derive(Copy, Clone, PartialEq, Debug, BitfieldSpecifier)]
#[bits = 1]
pub enum ResetDelay {
    Ms11 = 0b0,
    Ms90 = 0b1,
}

/// Reset delay
#[derive(Copy, Clone, PartialEq, Debug, BitfieldSpecifier)]
#[bits = 1]
pub enum OpampMuxMeasurement {
    BatteryVoltage = 0b0,
    Temperature = 0b1,
}

/// CONTROL1
#[cfg(tps_model = "TPS657201")]
#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Control1 {
    /// RESET_DELAY
    pub reset_delay: ResetDelay,

    /// OPAMP_EN
    pub opamp_mux_enabled: bool,

    /// OPAMP_MUX
    pub opamp_mux_measurement: OpampMuxMeasurement,

    /// B3 not used
    #[skip]
    pub __: B1,

    /// PB_STAT
    pub push_button_pressed: bool,

    /// HOLD
    #[skip(setters)]
    pub dcdc_ldo_enabled: bool,

    /// B6-7 not used
    #[skip]
    pub __: B2,
}

#[cfg(not(tps_model = "TPS657201"))]
#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Control1 {
    /// RESET_DELAY
    pub reset_delay: ResetDelay,

    /// B1-3 not used
    #[cfg(not(tps_model = "TPS657201"))]
    #[skip]
    pub __: B3,

    /// PB_STAT
    pub push_button_pressed: bool,

    /// HOLD
    #[skip(setters)]
    pub dcdc_ldo_enabled: bool,

    /// B6-7 not used
    #[skip]
    pub __: B2,
}

/// GPIO mode
#[derive(Copy, Clone, PartialEq, Debug, BitfieldSpecifier)]
#[bits = 1]
pub enum GPIOPull {
    PullLowLedDriver = 0b0,
    HighImpedence = 0b1,
}

#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct GPIOSSC {
    /// GPIO0
    pub gpio0: GPIOPull,

    /// GPIO1
    #[skip(setters)]
    pub gpio1: GPIOPull,

    /// GPIO2
    #[skip(setters)]
    pub gpio2: GPIOPull,

    /// GPIO3
    #[skip(setters)]
    pub gpio3: GPIOPull,

    /// B4-7 not used
    #[skip]
    pub __: B4,
}

/// GPIO input/output mode
#[derive(Copy, Clone, PartialEq, Debug, BitfieldSpecifier)]
#[bits = 1]
pub enum GPIOMode {
    Ouput = 0b0,
    Input = 0b1,
}

/// GPIO drive
#[derive(Copy, Clone, PartialEq, Debug, BitfieldSpecifier)]
#[bits = 1]
pub enum GPIODrive {
    StandardGPIO = 0b0,
    LedDriver5mA = 0b1,
}

#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct GPIOConfigControl {
    /// GPIO0_DIR
    pub gpio0: GPIOMode,

    /// GPIO1_DIR
    pub gpio1: GPIOMode,

    /// GPIO2_DIR
    pub gpio2: GPIOMode,

    /// GPIO3_DIR
    pub gpio3: GPIOMode,

    /// B4-5 not used
    #[skip]
    pub __: B2,

    /// GPIO2_LED
    pub gpio2_led: GPIODrive,

    /// GPIO3_LED
    pub gpio3_led: GPIODrive,
}

#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct IterruptMask0 {
    /// M_THLOOP
    pub thermal_loop_interrupt: bool,

    /// M_VBAT_COMP
    pub batt_voltage_comparator_interrupt: bool,

    /// M_CH_PGOOD
    pub power_source_ok_interrupt: bool,

    /// M_CH_ACTIVE
    pub charger_active: bool,

    /// Bit 4 skipped
    #[skip]
    pub __: B1,

    /// M_OVP
    pub over_voltage_protection_interrupt: bool,

    /// M_TS_COLD
    pub temp_low_interrupt: bool,

    /// M_TS_HOT
    pub temp_high_interrupt: bool,
}

/// IRMASK1 register
#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct InterruptMask1 {
    /// M_CH_SUSP
    pub suspended_interrupt: bool,

    /// M_CH_FAULT
    pub fault_interrupt: bool,

    /// M_CH_LDO
    pub ldo_interrupt: bool,

    /// M_CH_CC_CV
    pub constant_current_interrupt: bool,

    /// M_CH_PRECH
    pub precharge_interrupt: bool,

    /// M_CH_IDLE
    pub idle_interrupt: bool,

    /// M_CH_RESET
    pub reset_interrupt: bool,

    /// M_CH_SLEEP
    pub sleep_interrupt: bool,
}

/// IRMASK2 register
#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct InterruptMask2 {
    /// B0 skipped
    #[skip]
    pub __: B1,

    /// M_PB_STAT
    pub push_button_pressed_interrupt: bool,

    /// M_PGOODZ_LDO1
    pub ldo_power_source_ok_interrupt: bool,

    /// M_PGOODZ_DCDC
    pub dcdc_power_source_ok_interrupt: bool,

    /// M_GPIO0
    pub gpio0_interrupt: bool,

    /// M_GPIO1
    pub gpio1_interrupt: bool,

    /// M_GPIO2
    pub gpio2_interrupt: bool,

    /// M_GPIO3
    pub gpio3_interrupt: bool,
}

/// IR0 register
#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Iterrupt0 {
    /// THLOOP
    #[skip(setters)]
    pub thermal_loop: bool,

    /// VBAT_COMP
    #[skip(setters)]
    pub batt_voltage_comparator: bool,

    /// CH_PGOOD
    #[skip(setters)]
    pub power_source_ok: bool,

    /// CH_ACTIVE
    #[skip(setters)]
    pub charger_active: bool,

    /// Bit 4 skipped
    #[skip]
    pub __: B1,

    /// OVP
    #[skip(setters)]
    pub over_voltage_protection: bool,

    /// TS_COLD
    #[skip(setters)]
    pub temp_low: bool,

    /// TS_HOT
    #[skip(setters)]
    pub temp_high: bool,
}

/// IR1 register
#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Interrupt1 {
    /// CH_SUSP
    #[skip(setters)]
    pub suspended: bool,

    /// CH_FAULT
    #[skip(setters)]
    pub fault: bool,

    /// CH_LDO
    #[skip(setters)]
    pub ldo: bool,

    /// CH_CC_CV
    #[skip(setters)]
    pub constant_current: bool,

    /// CH_PRECH
    #[skip(setters)]
    pub precharge: bool,

    /// CH_IDLE
    #[skip(setters)]
    pub idle: bool,

    /// CH_RESET
    #[skip(setters)]
    pub reset: bool,

    /// CH_SLEEP
    #[skip(setters)]
    pub sleep: bool,
}

/// IR2 register
#[bitfield]
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Interrupt2 {
    /// B0 skipped
    #[skip]
    pub __: B1,

    /// PB_STAT
    #[skip(setters)]
    pub push_button_pressed: bool,

    /// PGOODZ_LDO1
    #[skip(setters)]
    pub ldo_power_source_ok: bool,

    /// PGOODZ_DCDC
    #[skip(setters)]
    pub dcdc_power_source_ok: bool,

    /// GPIO0
    #[skip(setters)]
    pub gpio0: bool,

    /// GPIO1
    #[skip(setters)]
    pub gpio1: bool,

    /// GPIO2
    #[skip(setters)]
    pub gpio2: bool,

    /// GPIO3
    #[skip(setters)]
    pub gpio3: bool,
}
