use modular_bitfield::prelude::*;

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum RegisterAddress {
    /// CHGSTATUS
    BatteryChargerStatus = 0x01,

    /// CHGCONFIGG0
    BatteryChargerConfigControl0 = 0x02,

    /// CHGCONFIGG1
    BatteryChargerConfigControl1 = 0x03,

    /// CHGCONFIGG2
    BatteryChargerConfigControl2 = 0x04,

    /// CHGCONFIGG3
    BatteryChargerConfigControl3 = 0x05,

    /// CHGSTATE
    BatteryChargeState = 0x06,

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
    pub thermal_loop_active: bool,

    /// CH_PGOOD
    pub power_source_ok: bool,

    /// CH_ACTIVE
    pub charger_active: bool,

    /// Bit 4 skipped
    #[skip]
    pub __: B1,

    /// OVP
    pub over_voltage_protection: bool,

    /// TS_COLD
    pub temp_low: bool,

    /// TS_HOT
    pub temp_high: bool,
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
    pub suspended: bool,

    /// CH_FAULT
    pub fault: bool,

    /// CH_LDO
    pub ldo: bool,

    /// CH_CC_CV
    pub constant_current: bool,

    /// CH_PRECH
    pub precharge: bool,

    /// CH_IDLE
    pub idle: bool,

    /// CH_RESET
    pub reset: bool,

    /// CH_SLEEP
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
    V1_625 = 33,
    V1_650 = 34,
    V1_675 = 35,
    V1_700 = 36,
    V1_725 = 37,
    V1_750 = 38,
    V1_775 = 39,
    V1_800 = 40,
    V1_825 = 41,
    V1_850 = 42,
    V1_875 = 43,
    V1_900 = 44,
    V1_925 = 45,
    V1_950 = 46,
    V1_975 = 47,
    V2_000 = 48,
    V2_025 = 49,
    V2_050 = 50,
    V2_075 = 51,
    V2_100 = 52,
    V2_125 = 53,
    V2_150 = 54,
    V2_175 = 55,
    V2_200 = 56,
    V2_225 = 57,
    V2_250 = 58,
    V2_275 = 59,
    V2_300 = 60
    V2_325 = 0b010010,
    V2_350 = 0b010010,
    V2_375 = 0b010010,
    V2_400 = 0b010010,
    V2_425 = 0b010010,
    V2_450 = 0b010010,
    V2_475 = 0b010010,
    V2_500 = 0b010010,
    V2_525 = 0b010010,
    V2_550 = 0b010010,
    V2_575 = 0b010010,
    V2_600 = 0b010010,
    V2_625 = 0b010010,
    V2_650 = 0b010010,
    V2_675 = 0b010010,
    V2_700 = 0b010010,
    V2_725 = 0b010010,
    V2_750 = 0b010010,
    V2_775 = 0b010010,
    V2_800 = 0b010010,
    V2_825 = 0b010010,
    V2_850 = 0b010010,
    V2_875 = 0b010010,
    V2_900 = 0b010010,
    V2_925 = 0b010010,
    V2_950 = 0b010010,
    V2_975 = 0b010010,
    V3_000 = 0b010010,
    V3_025 = 0b010010,
    V3_050 = 0b010010,
    V3_075 = 0b010010,
    V3_100 = 0b010010,
    V3_125 = 0b010010,
    V3_150 = 0b010010,
    V3_175 = 0b010010,
    V3_200 = 0b010010,
    V3_225 = 0b010010,
    V3_250 = 0b010010,
    V3_275 = 0b010010,
    V3_300 = 0b010010,
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
