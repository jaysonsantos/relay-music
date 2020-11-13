use stm32f1xx_hal::time::Hertz;

use defmt::Format;

use embedded_hal::timer::CountDown;
use embedded_hal::{digital::v2::OutputPin, timer::Periodic};

use crate::notes::{Duration, Note};
#[derive(Format)]
pub enum NotePlayerStatus {
    Start,
    Up,
    Down,
}

pub struct NotePlayer<'a, O, C>
where
    O: OutputPin,
    C: CountDown + Periodic,
{
    notes: &'a [(Note, Duration)],
    pin: O,
    note_timer: C,
    duration_counter: u8,
    current_note_idx: usize,
    status: NotePlayerStatus,
}

impl<'a, O, C> NotePlayer<'a, O, C>
where
    O: OutputPin,
    C: CountDown + Periodic,
    <C as CountDown>::Time: From<Hertz>,
{
    pub fn new(notes: &'a [(Note, Duration)], pin: O, note_timer: C) -> Self {
        Self {
            notes,
            pin,
            note_timer,
            duration_counter: 0,
            current_note_idx: 0,
            status: NotePlayerStatus::Start,
        }
    }

    pub fn tick(&mut self, has_duration_wrapped: bool) {
        let (note, duration) = self.get_current_note();
        self.increment_note_if_timer_wrapped(duration, has_duration_wrapped);

        match self.status {
            NotePlayerStatus::Start => {
                self.note_timer
                    .start::<Hertz>(note.note_to_relay_frequence());
                let _ = self.pin.set_high();
                self.status = NotePlayerStatus::Up;
            }
            NotePlayerStatus::Up => {
                if self.note_timer.wait().is_ok() {
                    let _ = self.pin.set_low();
                    self.status = NotePlayerStatus::Down;
                }
            }
            NotePlayerStatus::Down => {
                if self.note_timer.wait().is_ok() {
                    let _ = self.pin.set_high();
                    self.status = NotePlayerStatus::Up;
                }
            }
        }
    }

    fn get_current_note(&self) -> &'a (Note, Duration) {
        unsafe { self.notes.get_unchecked(self.current_note_idx) }
    }

    pub fn increment_note_if_timer_wrapped(
        &mut self,
        duration: &Duration,
        has_duration_wrapped: bool,
    ) {
        match self.status {
            NotePlayerStatus::Start => return,
            _ => (),
        };
        if has_duration_wrapped {
            if self.duration_counter == duration.ticks() {
                self.increment_note();
                self.duration_counter = 0;
            } else {
                self.duration_counter += 1;
            }
        }
    }

    pub fn increment_note(&mut self) {
        self.current_note_idx = (self.current_note_idx + 1) % self.notes.len();
        self.status = NotePlayerStatus::Start;
    }
}
