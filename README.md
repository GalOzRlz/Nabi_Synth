A [FunDSP](https://github.com/SamiPerttu/fundsp/tree/master) based synth-station that combines sound engines and effects into one affordable and infinitely expandible package. 

Currently holding one engine for Waveguide style synthesis on top of the [Midi_fundsp](https://github.com/gjf2a/midi_fundsp) framework.

The endgame is a stereo synth running on an off-the-shelf open-source SBC connected to your midi controller of choice.

Future plans are:
* ~~adding real time controls over midi cc~~
* support for saving and loading presets based on CC value
* support for more stereo manipulations (some already implemented)
* more synthesis engines and instruments to choose from (including classic analog ones)
* save your instruments state after manipulations with CC
* library of ready-to-drop effects for easier instrument creation via graphs
* global synth effects chain (midi fundsp feature)
* global LFO and/or ADSR envelope (midi fundsp feature)
* streamlined midi control over 0-8 params of each instruments with configurable mapping
* support for external screen showing instrument and impact of CC change?
