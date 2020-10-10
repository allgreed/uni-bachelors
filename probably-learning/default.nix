let
  pkgs = import ./pkgs.nix;
in
pkgs.mkShell {
  buildInputs =
  with pkgs;
  [
    R
    git
    #gnumake
    #entr
  ];
}
