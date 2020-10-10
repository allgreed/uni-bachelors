{ pkgs ? import <nixpkgs> {} }:
with pkgs;

let
in
mkShell {
  buildInputs = [ cargo rustup git gnumake ];
}
