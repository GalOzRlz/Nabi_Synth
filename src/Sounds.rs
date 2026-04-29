use fundsp::adsr::adsr_live;
use fundsp::prelude::{brown, db_amp, dcblock, join, limiter, lowpass_hz, mul, pass, resonator_hz, white, AudioUnit, U2};
use fundsp::prelude64::{add, clip, constant, highpass, highpass_hz, lowpass, lowpass_q, lowpole, resonator, sine, sine_hz, stack};
use midi_fundsp::sound_builders::{Adsr, ProgramTable, };
use midi_fundsp::{program_table, SharedMidiState};
use std::sync::Arc;
use fundsp::math::xerp;

mod instruments;

/// Returns a `ProgramTable` containing sounds that are personal favorites of the crate author.
pub fn favorites() -> ProgramTable {
    program_table![
        ("Yavin-Plucked", music_box)

    ]
}

pub fn music_box(state: &SharedMidiState) -> Box<dyn AudioUnit> {
    let synth_adsr = Adsr {
        attack: 0.002,
        decay: 0.0,
        sustain: 1.0,
        release: 0.6,
    };
    let a1 = 0.600;
    let a2 = 0.350;
    let a3 = 0.200;
    let a4 = 0.100;
    let gate = state.control_var();

    let modes = (mul(1.000) >> sine() * (gate.clone() >> adsr_live(0.002, 0.5, 0.0, synth_adsr.release))
        & (mul(2.756) >> sine() * a1) * (gate.clone() >> adsr_live(0.002, 0.5 / 2.0, 0.0, synth_adsr.release))
        & (mul(5.404) >> sine() * a2) * (gate.clone() >> adsr_live(0.002, 0.5 / 5.0, 0.0, synth_adsr.release))
        & (mul(8.933) >> sine() * a3) * (gate.clone() >> adsr_live(0.002, 0.5 / 10.0, 0.0, synth_adsr.release))
        & (mul(13.34) >> sine() * a4) * (gate.clone() >> adsr_live(0.002, 0.5 / 18.0, 0.0, synth_adsr.release))
        >> lowpass_hz(9000.0, 0.7))
        >> dcblock::<f64>();

    let tone = modes >> (pass() ^ (highpass_hz(100.0, 0.7) * 0.10)) >> join::<U2>();
    let body = (pass() * 0.7)
        & (0.5 * resonator_hz(150.0, 20.0))
        & (0.3 * resonator_hz(320.0, 25.0))
        & (0.1 * resonator_hz(550.0, 15.0));

    let cutoff_freq_32 = state.get_control_change(74).value() / 127.0;
    let cutoff_freq_signal = state.control_change_var(74)
        >> mul(xerp(20.0, 20_000.0, cutoff_freq_32));
    let combined = tone >> body * db_amp(-4.0) >> highpass_hz(30.0, 0.7);

    let synth = Box::new(
        stack(combined, cutoff_freq_signal)
            >> lowpass_q(0.5)     // This is the dynamic filter!
            >> highpass_hz(30.0, 0.7)  // You might want to reorder filters
            >> dcblock::<f64>()
            >> clip(),
    );
    state.assemble_unpitched_sound(synth, synth_adsr.boxed(state))
}