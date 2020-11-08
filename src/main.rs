#![no_std]
#![no_main]

use panic_probe as _; // global logger

use core::sync::atomic::{AtomicUsize, Ordering};

use cortex_m_rt::entry;

use nb::block;
use stm32f1xx_hal::{pac, prelude::*, time::Hertz, timer::Timer};

use defmt::info;
use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;

#[defmt::timestamp]
fn timestamp() -> u64 {
    static COUNT: AtomicUsize = AtomicUsize::new(0);
    // NOTE(no-CAS) `timestamps` runs with interrupts disabled
    let n = COUNT.load(Ordering::Relaxed);
    COUNT.store(n + 1, Ordering::Relaxed);
    n as u64
}

/// Terminates the application and makes `probe-run` exit with exit-code = 0
pub fn exit() -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

lazy_static::lazy_static! {
    static ref C: Hertz = 262.hz();
    static ref D: Hertz = 294.hz();
    static ref E: Hertz = 330.hz();
    static ref F: Hertz = 350.hz();
    static ref G: Hertz = 393.hz();
}

#[entry]
fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = dp.FLASH.constrain();
    // RCC = Reset and Clock Control
    let mut rcc = dp.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store
    // the frozen frequencies in `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Acquire the GPIO peripherals that we'll use
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    // Configure gpio B pin 12 as a push-pull output. The `crh` register is passed to the function
    // in order to configure the port. For pins 0-7, crl should be passed instead.
    let mut relay = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);

    // Configure the syst timer to trigger an update every second
    let mut note_timer = Timer::tim1(dp.TIM1, &clocks, &mut rcc.apb2).start_count_down(1.hz());
    let mut duration_timer = Timer::syst(cp.SYST, &clocks).start_count_down(2.hz());

    let notes: [&Hertz; 16] = [
        &E, &E, &F, &G, &G, &F, &E, &D, &C, &C, &D, &E, &D, &D, &C, &C,
    ];

    loop {
        for note in notes.iter() {
            info!("Playing {:u32}", note.0);
            note_timer.start(**note);
            while let Err(nb::Error::WouldBlock) = duration_timer.wait() {
                block!(note_timer.wait()).unwrap();
                relay.set_high().unwrap();
                block!(note_timer.wait()).unwrap();
                relay.set_low().unwrap();
            }
        }
    }
}
