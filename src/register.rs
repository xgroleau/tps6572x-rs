#[repr(u8)]
pub enum Register {
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
