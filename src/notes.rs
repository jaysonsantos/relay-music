use stm32f1xx_hal::{prelude::*, time::Hertz};

#[allow(dead_code)]
pub enum Note {
    C(usize),
    D(usize),
    E(usize),
    F(usize),
    Fs(usize),
    G(usize),
    A(usize),
    B(usize),
}

impl Note {
    pub fn note_to_relay_frequence(&self) -> Hertz {
        let frequence = match self {
            Note::C(octave) => match octave {
                3 => 330,
                4 => 523,
                5 => 1045,
                octave => todo!("Implement {}", octave),
            },
            Note::D(octave) => match octave {
                3 => 294,
                4 => 587,
                5 => 1174,
                octave => todo!("Implement {}", octave),
            },
            Note::E(octave) => match octave {
                3 => 330,
                4 => 659,
                octave => todo!("Implement {}", octave),
            },
            Note::F(octave) => match octave {
                3 => 350,
                4 => 698,
                octave => todo!("Implement {}", octave),
            },
            Note::Fs(octave) => match octave {
                3 => 371,
                4 => 745,
                octave => todo!("Implement {}", octave),
            },
            Note::G(octave) => match octave {
                3 => 393,
                4 => 784,
                octave => todo!("Implement {}", octave),
            },
            Note::A(octave) => match octave {
                3 => 440, // Go figure why A3 and not A4
                4 => 880,
                octave => todo!("Implement {}", octave),
            },
            Note::B(octave) => match octave {
                3 => 394,
                4 => 989,
                octave => todo!("Implement {}", octave),
            },
        };
        frequence.hz()
    }
}

pub enum Duration {
    Whole,
    Half,
    OneFourth,
    OneEigth,
    OneSixteenth,
}

impl Duration {
    pub fn ticks(&self) -> u8 {
        match self {
            Duration::Whole => 16,
            Duration::Half => 8,
            Duration::OneFourth => 4,
            Duration::OneEigth => 2,
            Duration::OneSixteenth => 1,
        }
    }
}
