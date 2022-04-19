#[repr(u8)]
pub enum Register {
    BatteryChargerStatus = 0x01,
    BatteryChargerConfigControl0 = 0x02,
    BatteryChargerConfigControl1 = 0x03,
    BatteryChargerConfigControl2 = 0x04,
    BatteryChargerConfigControl3 = 0x05,
    BatteryChargeState = 0x06,
    DCDCSettingControl = 0x07,
    LDOSettingControl = 0x08,
    Control0 = 0x09,
    Control1 = 0x0A,
    GPIOSCC = 0x0B,
    GPIOConfigControl = 0x0C,
    InterruptMask0 = 0xD0,
    InterruptMask1 = 0xE0,
    InterruptMask2 = 0xF0,
    InterruptReporting0 = 0x10,
    InterruptReporting1 = 0x11,
    InterruptReporting2 = 0x12,

}