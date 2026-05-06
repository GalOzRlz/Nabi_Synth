use fundsp::adsr::adsr_live;
use fundsp::prelude::{brown, db_amp, dcblock, join, limiter, lowpass_hz, mul, pass, resonator_hz, white, AudioUnit, U2};
use fundsp::prelude64::{clip, constant, highpass, highpass_hz, resonator, sine, sine_hz};
use midi_fundsp::sound_builders::*;
use midi_fundsp::{program_table, SharedMidiState};
mod instruments;
use instruments::*;

/// Returns a `ProgramTable` containing sounds that are personal favorites of the crate author.
pub fn favorites() -> ProgramTable {
    program_table![
        ("Modal Plucked string", modal_plucked_string),
        ("Bell", modal_bell),
        ("Music Box", modal_struck_bar),
        ("Egogo-Drum", modal_membrane),
        ("Modal Airy Pad", breathy),
        ("Waveguide Harpsichord", harpsichord),
        ("Plastic Pipe", plastic_pipe),
        ("stereo Guitar", (dirty_guitar_stereo_l, dirty_guitar_stereo_r))
    ]
}

pub fn modal_plucked_string(state: &SharedMidiState) -> Box<dyn AudioUnit> {
    let synth_adsr = Adsr {
        attack: 0.002,
        decay: 0.0,
        sustain: 1.0,
        release: 0.01,
    };
    let a1 = 0.500;
    let a2 = 0.333;
    let a3 = 0.250;
    let a4 = 0.200;
    let a5 = 0.167;
    let a6 = 0.143;
    let a7 = 0.125;
    let gate = state.control_var();

    let modes = (mul(1.0)
        >> sine() * (gate.clone() >> adsr_live(0.002, 0.5 / 0.3, 0.0, synth_adsr.release))
        & (mul(2.001) >> sine() * a1)
            * (gate.clone() >> adsr_live(0.0001, 0.5 / 0.8, 0.0, synth_adsr.release))
        & (mul(3.004) >> sine() * a2)
            * (gate.clone() >> adsr_live(0.0001, 0.5 / 1.5, 0.0, synth_adsr.release))
        & (mul(4.010) >> sine() * a3)
            * (gate.clone() >> adsr_live(0.0001, 0.5 / 2.5, 0.0, synth_adsr.release))
        & (mul(5.020) >> sine() * a4)
            * (gate.clone() >> adsr_live(0.0001, 0.5 / 4.0, 0.0, synth_adsr.release))
        & (mul(6.035) >> sine() * a5)
            * (gate.clone() >> adsr_live(0.0001, 0.5 / 6.0, 0.0, synth_adsr.release))
        & (mul(7.055) >> sine() * a6)
            * (gate.clone() >> adsr_live(0.0001, 0.5 / 8.5, 0.0, synth_adsr.release))
        & (mul(8.080) >> sine() * a7)
            * (gate.clone() >> adsr_live(0.0001, 0.5 / 12.0, 0.0, synth_adsr.release))
            >> highpass_hz(40.0, 0.6)
            >> lowpass_hz(4000.0, 0.7))
        >> dcblock::<f64>() * 0.5
        >> clip();

    let tone = modes >> (pass() ^ (highpass_hz(100.0, 0.7) * 0.10)) >> join::<U2>();

    let synth =
        Box::new(tone >> highpass_hz(30.0, 0.7) >> dcblock::<f64>() >> limiter(0.002, 0.12));

    state.assemble_unpitched_sound(synth, synth_adsr.boxed(state))
}

pub fn modal_bell(state: &SharedMidiState) -> Box<dyn AudioUnit> {
    let synth_adsr = Adsr {
        attack: 0.02,
        decay: 0.0,
        sustain: 1.0,
        release: 0.5,
    };
    let a1 = 0.700;
    let a2 = 0.500;
    let a3 = 0.300;
    let a4 = 0.150;
    let a5 = 0.080;
    let gate = state.control_var();

    let modes = (mul(1.000)
        >> sine() * (gate.clone() >> adsr_live(0.002, 0.6 / 0.5, 0.0, synth_adsr.release))
        & (mul(2.143) >> sine() * a1)
            * (gate.clone() >> adsr_live(0.002, 0.6 / 3.0, 0.0, synth_adsr.release))
        & (mul(3.413) >> sine() * a2)
            * (gate.clone() >> adsr_live(0.002, 0.6 / 6.0, 0.0, synth_adsr.release))
        & (mul(4.090) >> sine() * a3)
            * (gate.clone() >> adsr_live(0.002, 0.6 / 10.0, 0.0, synth_adsr.release))
        & (mul(5.190) >> sine() * a4)
            * (gate.clone() >> adsr_live(0.002, 0.6 / 15.0, 0.0, synth_adsr.release))
        & (mul(6.250) >> sine() * a5)
            * (gate.clone() >> adsr_live(0.002, 0.6 / 20.0, 0.0, synth_adsr.release))
            >> lowpass_hz(6000.0, 0.7))
        >> dcblock::<f64>();

    let tone = modes >> (pass() ^ (highpass_hz(100.0, 0.7) * 0.10)) >> join::<U2>();

    let synth = Box::new(
        tone * db_amp(-4.0) >> highpass_hz(100.0, 0.7) >> dcblock::<f64>() >> limiter(0.002, 0.12),
    );

    state.assemble_unpitched_sound(synth, synth_adsr.boxed(state))
}

pub fn modal_struck_bar(state: &SharedMidiState) -> Box<dyn AudioUnit> {
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

    // Rates for struck bar: 0.4, 2.0, 5.0, 10.0, 18.0
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
            >> lowpass_hz(5000.0, 0.7))
        >> dcblock::<f64>();

    let tone = modes >> (pass() ^ (highpass_hz(100.0, 0.7) * 0.10)) >> join::<U2>();

    let body = (pass() * 0.7)
        & (0.5 * resonator_hz(150.0, 20.0))
        & (0.3 * resonator_hz(320.0, 25.0))
        & (0.1 * resonator_hz(550.0, 15.0));

    let synth = Box::new(
        tone >> body * db_amp(-4.0)
            >> highpass_hz(30.0, 0.7)
            >> dcblock::<f64>()
            >> limiter(0.002, 0.12),
    );

    state.assemble_unpitched_sound(synth, synth_adsr.boxed(state))
}

// -----------------------------------------------------------------------------
// Modal Membrane / Drum (adapted from the same pattern)
// -----------------------------------------------------------------------------
pub fn modal_membrane(state: &SharedMidiState) -> Box<dyn AudioUnit> {
    let synth_adsr = Adsr {
        attack: 0.0001,
        decay: 0.2,
        sustain: 0.0,
        release: 0.01,
    };
    let a1 = 0.700;
    let a2 = 0.500;
    let a3 = 0.400;
    let a4 = 0.300;

    let decay_base = 0.7;
    let gate = state.control_var();

    // Rates for membrane: 0.2, 0.8, 1.8, 3.0, 5.0
    let modes = (mul(1.000)
        >> sine() * (gate.clone() >> adsr_live(0.002, decay_base / 0.2, 0.0, synth_adsr.release))
        & (mul(1.593) >> sine() * a1)
            * (gate.clone() >> adsr_live(0.002, decay_base / 0.8, 0.0, synth_adsr.release))
        & (mul(2.135) >> sine() * a2)
            * (gate.clone() >> adsr_live(0.002, decay_base / 1.8, 0.0, synth_adsr.release))
        & (mul(2.296) >> sine() * a3)
            * (gate.clone() >> adsr_live(0.002, decay_base / 3.0, 0.0, synth_adsr.release))
        & (mul(2.917) >> sine() * a4)
            * (gate.clone() >> adsr_live(0.002, decay_base / 5.0, 0.0, synth_adsr.release))
            >> lowpass_hz(4000.0, 0.6))
        >> dcblock::<f64>();

    let tone = modes >> (pass() ^ (highpass_hz(100.0, 0.7) * 0.10)) >> join::<U2>();

    let synth =
        Box::new(tone >> highpass_hz(30.0, 0.7) >> dcblock::<f64>() >> limiter(0.002, 0.12));

    state.assemble_unpitched_sound(synth, synth_adsr.boxed(state))
}

pub fn breathy(state: &SharedMidiState) -> Box<dyn AudioUnit> {
    let external_adsr = Adsr {
        attack: 0.15,
        decay: 0.1,
        sustain: 0.7,
        release: 0.001,
    };

    let gate = state.control_var();

    let breath = white() * 0.5 * gate.clone();

    let base_pitch = state.bent_pitch();
    let lfo = sine_hz(3.0) * 0.0065;
    let pitch = base_pitch * (constant(1.0) + lfo);

    let attack = brown::<f64>() * (gate.clone() >> adsr_live(0.01, 0.01, 0.00, 0.001));
    let pipe1 = (breath.clone() | pitch.clone() | constant(95.0)) >> resonator();
    let pipe2 = (pipe1.clone() | pitch.clone() * constant(2.015) | constant(75.0))
        >> resonator() * constant(0.6);
    let pipe3 = (pipe2.clone() | pitch.clone() * constant(3.120) | constant(75.0))
        >> resonator() * constant(0.2);
    let pipe4 = (pipe2.clone() | pitch.clone() * constant(4.215) | constant(75.0))
        >> resonator() * constant(0.10);
    let pipe5 = (pipe2.clone() | pitch.clone() * constant(5.320) | constant(65.0))
        >> resonator() * constant(0.05);
    // Post‑processing (same as other instruments)
    let processed = ((attack & pipe1 & pipe2 & pipe3 & pipe4 & pipe5) * constant(0.5)
        | pitch.clone()
        | constant(0.5))
        >> highpass()
        >> lowpass_hz(5000.0, 0.1)
        >> dcblock::<f64>();

    let synth = Box::new(processed >> highpass_hz(60.0, 0.9) >> dcblock::<f64>() >> clip());

    state.assemble_pitched_sound(synth, external_adsr.boxed(state))
}

pub fn harpsichord(state: &SharedMidiState) -> Box<dyn AudioUnit> {
    let adsr = Adsr {
        attack: 0.005,
        decay: 0.8,
        sustain: 0.0,
        release: 0.0,
    };
    let gate = state.control_var().clone();
    let mix = (state.bent_pitch().clone() | gate | constant(0.0))
        >> pluck_comb_string()
        >> lowpass_hz(9000.0, 0.5);
    state.assemble_pitched_sound(Box::new(mix), adsr.boxed(state))
}

pub fn plastic_pipe(state: &SharedMidiState) -> Box<dyn AudioUnit> {
    let adsr = Adsr {
        attack: 0.005,
        decay: 0.5,
        sustain: 0.0,
        release: 0.0,
    };
    let gate = state.control_var().clone();
    let mix = (state.bent_pitch().clone() | gate | constant(0.0))
        >> hit_comb_pipe()
        >> lowpass_hz(7000.0, 0.5);
    state.assemble_pitched_sound(Box::new(mix), adsr.boxed(state))
}

pub fn dirty_guitar_stereo_l(state: &SharedMidiState) -> Box<dyn AudioUnit> {
    let adsr = Adsr {
        attack: 0.005,
        decay: 0.8,
        sustain: 1.0,
        release: 0.2,
    };
    let base_pitch = state.bent_pitch();
    let lfo1 = sine_hz(3.0) * 0.0165;
    let pitch1 = base_pitch.clone() * (constant(1.0) + lfo1);
    let gate = state.control_var();
    let dg = dirty_guitar();
    state.assemble_pitched_sound(Box::new(dg(pitch1, gate.clone())), adsr.boxed(state))
}

pub fn dirty_guitar_stereo_r(state: &SharedMidiState) -> Box<dyn AudioUnit> {
    let adsr = Adsr {
        attack: 0.005,
        decay: 0.85,
        sustain: 0.9,
        release: 0.15,
    };
    let base_pitch = state.bent_pitch();
    let lfo2 = sine_hz(3.5) * 0.0065;
    let pitch2 = base_pitch.clone() * (constant(1.0) + lfo2);
    let gate = state.control_var();
    let dg = dirty_guitar();
    state.assemble_pitched_sound(Box::new(dg(pitch2, gate.clone())), adsr.boxed(state))
}
