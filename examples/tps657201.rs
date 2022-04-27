#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy::{
    executor::Spawner,
};
use embassy_nrf::{
    interrupt,
    twim, Peripherals,
};
use panic_probe as _;
use tps6572x::{TPS6572x, registers::*};


#[embassy::main]
async fn main(_spawner: Spawner, p: Peripherals) {
    let config = twim::Config::default();
    let irq = interrupt::take!(SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0);

    let twi = twim::Twim::new(p.TWISPI0, irq, p.P1_15, p.P0_15, config);

    let mut tps = TPS6572x::new(twi);

    let cfg0_initial = tps.read_register::<ChargerConfig0>().unwrap();

    let mut cfg0 = cfg0_initial.clone();
    cfg0.set_charger_enabled(!cfg0.charger_enabled());

    tps.write_register(cfg0).unwrap();

    tps.edit_register::<ChargerConfig0, _>(|mut r| {
        assert!(cfg0 == r);
        r.set_charger_enabled(!r.charger_enabled());
        r
    }).unwrap();

    assert!(cfg0_initial == tps.read_register::<ChargerConfig0>().unwrap());
    

}