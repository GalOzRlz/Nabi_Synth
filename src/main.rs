use std::sync::{Arc, Mutex};
mod sounds;

use crossbeam_queue::SegQueue;
use crossbeam_utils::atomic::AtomicCell;
use midi_fundsp::{
    io::{
        Speaker, SynthMsg, console_choice_from, get_first_midi_device, start_input_thread,
        start_output_thread,
    },
    sound_builders::ProgramTable,
};
use midir::MidiInput;

fn main() -> anyhow::Result<()> {
    let reset = Arc::new(AtomicCell::new(false));
    let mut quit = false;
    while !quit {
        let mut midi_in = MidiInput::new("midir reading input")?;
        let in_port = get_first_midi_device(&mut midi_in)?;
        let midi_msgs = Arc::new(SegQueue::new());
        while reset.load() {}
        start_input_thread(midi_msgs.clone(), midi_in, in_port, reset.clone());
        let program_table = Arc::new(Mutex::new(sounds::favorites()));
        start_output_thread::<5>(midi_msgs.clone(), program_table.clone());
        run_chooser(midi_msgs, program_table.clone(), reset.clone(), &mut quit);
    }
    Ok(())
}

fn run_chooser(
    midi_msgs: Arc<SegQueue<SynthMsg>>,
    program_table: Arc<Mutex<ProgramTable>>,
    reset: Arc<AtomicCell<bool>>,
    quit: &mut bool,
) {
    let main_menu = vec!["Pick New Synthesizer Sound", "Pick New MIDI Device", "Quit"];
    while !*quit && !reset.load() {
        println!("Play notes at will. When ready for a change, select one of the following:");
        match console_choice_from("Choice", &main_menu, |s| *s) {
            0 => {
                let program = {
                    let program_table = program_table.lock().unwrap();
                    console_choice_from("Change synth to", &program_table.entries, |opt| opt.0.as_str())
                };
                midi_msgs.push(SynthMsg::program_change(program as u8, Speaker::Both));
            }
            1 => reset.store(true),
            2 => *quit = true,
            _ => panic!("This should never happen."),
        }
    }
}
