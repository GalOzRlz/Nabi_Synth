A [FunDSP](https://github.com/SamiPerttu/fundsp/tree/master) based synth specializing in physical modeling techniques.

Currently holding one engine for Waveguide style synthesis on top of the [Midi_fundsp](https://github.com/gjf2a/midi_fundsp) framework.
The endgame is a synth running on an SBC connected to your midi controller of choice.

Future plans are:
* ~~adding real time controls over midi cc~~
* support for more stereo manipulations (some already implemented)
* more synthesis engines and instruments to choose from (including classic analog ones)
* save your instruments state after manipulations with CC
* library of ready-to-drop effects for easier instrument creation via graphs
* streamlined midi control over 0-8 params of each instruments with configurable mapping
* support for external screen showing instrument and impact of CC change?
