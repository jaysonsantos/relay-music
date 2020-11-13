#![no_std]
#![no_main]

use cortex_m_rt::entry;

use stm32f1xx_hal::{pac, prelude::*, timer::Timer};

use embedded_hal::timer::CountDown;

use relay_music::{note_player::NotePlayer, songs::ode_to_joy};

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
    let channel1_relay = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);
    let channel2_relay = gpiob.pb13.into_push_pull_output(&mut gpiob.crh);
    let channel3_relay = gpiob.pb14.into_push_pull_output(&mut gpiob.crh);
    let channel4_relay = gpiob.pb15.into_push_pull_output(&mut gpiob.crh);

    // Configure the syst timer to trigger an update every second
    // let button_timer = Timer::syst(cp.SYST, &clocks).start_count_down(1.hz());
    let mut duration_timer = Timer::syst(cp.SYST, &clocks).start_count_down(9.hz());
    let channel1_note_timer = Timer::tim1(dp.TIM1, &clocks, &mut rcc.apb2).start_count_down(1.hz());
    let channel2_note_timer = Timer::tim2(dp.TIM2, &clocks, &mut rcc.apb1).start_count_down(1.hz());
    let channel3_note_timer = Timer::tim3(dp.TIM3, &clocks, &mut rcc.apb1).start_count_down(1.hz());
    let channel4_note_timer = Timer::tim4(dp.TIM4, &clocks, &mut rcc.apb1).start_count_down(1.hz());

    let (channel1_notes, channel2_notes, channel3_notes, channel4_notes) = ode_to_joy();
    let mut channel1_player = NotePlayer::new(&channel1_notes, channel1_relay, channel1_note_timer);
    let mut channel2_player = NotePlayer::new(&channel2_notes, channel2_relay, channel2_note_timer);
    let mut channel3_player = NotePlayer::new(&channel3_notes, channel3_relay, channel3_note_timer);
    let mut channel4_player = NotePlayer::new(&channel4_notes, channel4_relay, channel4_note_timer);

    loop {
        let duration_timer_wrapped = duration_timer.wait().is_ok();
        channel1_player.tick(duration_timer_wrapped);
        channel2_player.tick(duration_timer_wrapped);
        channel3_player.tick(duration_timer_wrapped);
        channel4_player.tick(duration_timer_wrapped);
    }
}
