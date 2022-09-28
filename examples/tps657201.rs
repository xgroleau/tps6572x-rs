#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::info;
use defmt_rtt as _;
use embassy::executor::Spawner;
use embassy_nrf::{interrupt, twim, Peripherals};
use panic_probe as _;
use tps6572x::{registers::*, TPS6572x};

#[embassy::main]
async fn main(_spawner: Spawner, p: Peripherals) {
    info!("Started test");
    let config = twim::Config::default();
    let irq = interrupt::take!(SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0);

    let twi = twim::Twim::new(p.TWISPI0, irq, p.P1_15, p.P0_15, config);

    let mut tps = TPS6572x::new(twi);

    // Read the register
    let cfg0: ChargerConfig0 = tps.read_register().unwrap();

    // Write the new value
    let test = ChargerConfig0::new();
    test.with_charger_enabled(true);
    tps.write_register(test).unwrap();

    // Read the new value, compare and revert the changes
    tps.edit_register::<ChargerConfig0, _>(|mut r| {
        assert!(cfg0 == r);
        r.set_charger_enabled(!r.charger_enabled());
        r
    })
    .unwrap();

    // Check that the value is now the initial value
    assert!(cfg0 == tps.read_register::<ChargerConfig0>().unwrap());

    info!("Example successfully completed");
    cortex_m::asm::bkpt();
}
