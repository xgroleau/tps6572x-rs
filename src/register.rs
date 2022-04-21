use num_enum::TryFromPrimitive;

#[derive(Copy, Clone, PartialEq, Debug, TryFromPrimitive)]
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
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ChargerStatus {
    /// TS_HOT
    pub temp_high: bool,

    /// TS_COLD
    pub temp_low: bool,

    /// OVP
    pub over_voltage_protection: bool,

    /// CH_ACTIVE
    pub charger_active: bool,

    /// CH_PGOOD
    pub power_source_ok: bool,

    /// CH_THLOOP
    pub thermal_loop_active: bool,
}

/// Represents the possible output voltage for the `[ChargerConfig0]`
#[derive(Copy, Clone, PartialEq, Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum OutputVoltage {
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
#[derive(Copy, Clone, PartialEq, Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum AcInputCurrent {
    /// 100mA, with DDPM
    mA100_DDPM = 0b00,

    /// 500mA, with DDPM
    mA500_DDPM = 0b01,

    /// 500mA, without DDPM
    mA500 = 0b10,

    /// Usb suspend mode, standby
    UsbSuspend = 0b11,
}

/// CHGCONFIG0 register
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ChargerConfig0 {
    pub output_voltage: OutputVoltage,
    pub ac_input_current: AcInputCurrent,
    pub thermal_loop: bool,
    pub dynamic_timer: bool,
    pub termination_enabled: bool,
    pub charger_enabled: bool,
}
