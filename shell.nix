{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.rustup
    pkgs.curl
    pkgs.gcc
    pkgs.alsa-lib
    pkgs.pkg-config
    pkgs.pipewire
  ];

  shellHook = ''
    export ALSA_PLUGIN_DIR=${pkgs.pipewire}/lib/alsa-lib
    export XDG_RUNTIME_DIR=''${XDG_RUNTIME_DIR:-/run/user/$(id -u)}
    export LD_LIBRARY_PATH=${pkgs.alsa-lib}/lib''${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}
  '';
}
