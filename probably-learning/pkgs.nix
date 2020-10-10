let
  nixpkgs = builtins.fetchGit {
    name = "nixos-unstable-2020-05-9";
    url = "https://github.com/nixos/nixpkgs-channels/";
    ref = "refs/heads/nixos-unstable";
    rev = "fce7562cf46727fdaf801b232116bc9ce0512049";
    # obtain via `git ls-remote https://github.com/nixos/nixpkgs-channels nixos-unstable`
  };
in
  import nixpkgs { config = {}; }
