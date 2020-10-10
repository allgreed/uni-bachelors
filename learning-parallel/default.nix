{ pkgs ? import <nixpkgs> {} }:
with pkgs;

let
in
mkShell {
  buildInputs = [ bc gcc git gnumake gdb valgrind xorg.libX11];
}
