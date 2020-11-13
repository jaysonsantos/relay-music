use crate::notes::{Duration, Note};

pub fn ode_to_joy() -> (
    [(Note, Duration); 15],
    [(Note, Duration); 5],
    [(Note, Duration); 5],
    [(Note, Duration); 5],
) {
    let channel1_notes = [
        (Note::B(4), Duration::OneFourth),
        (Note::B(4), Duration::OneFourth),
        (Note::C(5), Duration::OneFourth),
        (Note::D(5), Duration::OneFourth),
        (Note::D(5), Duration::OneFourth),
        (Note::C(5), Duration::OneFourth),
        (Note::B(4), Duration::OneFourth),
        (Note::A(4), Duration::OneFourth),
        (Note::G(4), Duration::OneFourth),
        (Note::G(4), Duration::OneFourth),
        (Note::A(4), Duration::OneFourth),
        (Note::B(4), Duration::OneFourth),
        (Note::B(4), Duration::OneEigth),
        (Note::A(4), Duration::OneEigth),
        (Note::A(4), Duration::Half),
    ];
    let channel2_notes = [
        (Note::D(4), Duration::Whole),
        (Note::D(4), Duration::Whole),
        (Note::D(4), Duration::Whole),
        (Note::D(4), Duration::Half),
        (Note::D(4), Duration::Half),
    ];
    let channel3_notes = [
        (Note::B(3), Duration::Whole),
        (Note::A(3), Duration::Whole),
        (Note::B(3), Duration::Whole),
        (Note::B(3), Duration::Half),
        (Note::A(3), Duration::Half),
    ];
    let channel4_notes = [
        (Note::G(3), Duration::Whole),
        (Note::Fs(3), Duration::Whole),
        (Note::G(3), Duration::Whole),
        (Note::G(3), Duration::Half),
        (Note::Fs(3), Duration::Half),
    ];

    (
        channel1_notes,
        channel2_notes,
        channel3_notes,
        channel4_notes,
    )
}
