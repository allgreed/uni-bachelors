{ pkgs ? import <nixpkgs> {} }:

with pkgs;

let
  dotnet = dotnet-sdk;
in
mkShell {
  buildInputs = [ dotnet git gnumake ];
}
