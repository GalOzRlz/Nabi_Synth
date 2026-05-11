A [FunDSP](https://github.com/SamiPerttu/fundsp/tree/master) based synth-station that combines sound engines and effects into one affordable and infinitely expandible package. 

Currently holding one engine for Waveguide style synthesis on top of the [Midi_fundsp](https://github.com/gjf2a/midi_fundsp) framework.

Our goal is creating a truely affordable (35-45 USD) no-solder stereo synth running on an off-the-shelf open-source SBC connected to your midi controller of choice.

Future plans are:
* ~~adding real time controls over midi cc~~
* full wiki manual that covers all the steps: from purchasing parts to bootstrapping the synth.
* support for saving and loading presets from the midi controller
* ~~support for more stereo manipulations (some already implemented)~~
* more synthesis engines and instruments to choose from (including classic analog ones)
* library of ready-to-drop effects and oscillators for easier instrument
* ~~master bus effects chain~~
* global LFO and/or ADSR envelope
* streamlined midi control ove 8 params of each instruments with configurable override-mapping via toml files
* on board web-based editor you can connect to manage your synth
* support for external screen showing current instrument and impact of CC change
