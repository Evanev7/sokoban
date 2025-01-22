{
  description = "see3-rs Nix Dev Flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/release-24.11";
  };

  outputs = inputs: let
    system = "x86_64-linux";
    pkgs = import inputs.nixpkgs {
      inherit system;
      config.allowUnfree = true;
    };
  in
    with pkgs; {
      devShells.${system}.default = mkShell {
        buildInputs = [
          rustc
          rust-analyzer
          clippy
          bacon
          cargo
        ];
      };
    };
}
