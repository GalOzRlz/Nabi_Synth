use fundsp::adsr::adsr_live;
use fundsp::prelude::{db_amp, dcblock, join, mul, pass, resonator_hz, AudioUnit, U2};
use fundsp::prelude64::{clip, follow, highpass_hz, lowpass_q, map, sine};
use midi_fundsp::sound_builders::{Adsr, ProgramTable};
use midi_fundsp::{program_table, SharedMidiState};
use std::sync::Arc;

mod instruments;

/// Returns a `ProgramTable` containing sounds that are personal favorites of the crate author.
pub fn favorites() -> ProgramTable {
    program_table![("Yavin-Plucked", music_box)]
}

/// Something between a celesta and a prepared-piano with filter cutoff mapped to midi CC 74
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

    let modes = (mul(1.000)
        >> sine() * (gate.clone() >> adsr_live(0.002, 0.5, 0.0, synth_adsr.release))
        & (mul(2.756) >> sine() * a1)
            * (gate.clone() >> adsr_live(0.002, 0.5 / 2.0, 0.0, synth_adsr.release))
        & (mul(5.404) >> sine() * a2)
            * (gate.clone() >> adsr_live(0.002, 0.5 / 5.0, 0.0, synth_adsr.release))
        & (mul(8.933) >> sine() * a3)
            * (gate.clone() >> adsr_live(0.002, 0.5 / 10.0, 0.0, synth_adsr.release))
        & (mul(13.34) >> sine() * a4)
            * (gate.clone() >> adsr_live(0.002, 0.5 / 18.0, 0.0, synth_adsr.release))
        >> dcblock::<f64>());

    let tone = modes >> (pass() ^ (highpass_hz(100.0, 0.7) * 0.10)) >> join::<U2>();
    let body = (pass() * 0.7)
        & (0.5 * resonator_hz(150.0, 20.0))
        & (0.3 * resonator_hz(320.0, 25.0))
        & (0.1 * resonator_hz(550.0, 15.0));

    // set cutoff stream with smoothing
    let cutoff_freq = state.control_change_var(74)
        >> map(|frame| {
            let min_freq = 100.0_f32;
            let max_freq = 17000.0_f32;
            let norm = frame[0] / 127.0;
            min_freq * max_freq / min_freq.powf(norm)
        })
        >> follow(0.05_f32); // smoothing

    let combined = tone >> body >> highpass_hz(30.0, 0.7) * db_amp(-4.0) ;

    let synth =
        Box::new((combined | cutoff_freq) >> lowpass_q(0.8) >> dcblock::<f64>() >> clip());
    state.assemble_unpitched_sound(synth, synth_adsr.boxed(state))
}
