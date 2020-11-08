#![no_std]
#![no_main]

use panic_probe as _; // global logger

use core::sync::atomic::{AtomicUsize, Ordering};

use cortex_m_rt::entry;

use stm32f1xx_hal::{pac, prelude::*, time::Hertz, timer::Timer};

use defmt::{info, Format};
use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::timer::CountDown;

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
#[derive(Format)]
enum NotePlayerStatus {
    Start,
    Up,
    Down,
}

struct NotePlayer<'a, O, C>
where
    O: OutputPin,
    C: CountDown,
{
    notes: &'a [&'static Hertz],
    pin: O,
    timer: C,
    current_note_idx: usize,
    status: NotePlayerStatus,
}

impl<'a, O, C> NotePlayer<'a, O, C>
where
    O: OutputPin,
    C: CountDown,
{
    pub fn new(notes: &'a [&'static Hertz], pin: O, timer: C) -> Self {
        Self {
            notes,
            pin,
            timer,
            current_note_idx: 0,
            status: NotePlayerStatus::Start,
        }
    }

    pub fn tick(&mut self)
    where
        <C as CountDown>::Time: From<Hertz>,
    {
        match self.status {
            NotePlayerStatus::Start => {
                self.timer.start::<Hertz>(*self.get_current_note());
                let _ = self.pin.set_high();
                self.status = NotePlayerStatus::Up;
            }
            NotePlayerStatus::Up => {
                if self.timer.wait().is_ok() {
                    let _ = self.pin.set_low();
                    self.status = NotePlayerStatus::Down;
                }
            }
            NotePlayerStatus::Down => {
                if self.timer.wait().is_ok() {
                    let _ = self.pin.set_high();
                    self.status = NotePlayerStatus::Up;
                }
            }
        }
    }

    fn get_current_note(&self) -> &'a Hertz {
        unsafe { self.notes.get_unchecked(self.current_note_idx) }
    }

    fn increment_note(&mut self) {
        self.current_note_idx = (self.current_note_idx + 1) % self.notes.len();
        self.status = NotePlayerStatus::Start;
    }
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
    let relay = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);

    // Configure the syst timer to trigger an update every second
    let note_timer = Timer::tim1(dp.TIM1, &clocks, &mut rcc.apb2).start_count_down(1.hz());
    let mut duration_timer = Timer::syst(cp.SYST, &clocks).start_count_down(2.hz());

    let notes: &[&Hertz] = &[
        &E, &E, &F, &G, &G, &F, &E, &D, &C, &C, &D, &E, &D, &D, &C, &C,
    ];
    let mut notes_player = NotePlayer::new(notes, relay, note_timer);

    loop {
        info!(
            "Playing {:u32} {:?}",
            notes_player.get_current_note().0,
            notes_player.status
        );
        while let Err(nb::Error::WouldBlock) = duration_timer.wait() {
            notes_player.tick();
        }
        notes_player.increment_note();
    }
}
