{
  description = "see3-rs Nix Dev Flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay, }: 
  let
    overlays = [ (import rust-overlay) ];
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system overlays;
      config.allowUnfree = true;
    };
  in
  with pkgs;
  {
    devShells.${system}.default = let
      build-rust = rust-bin.stable.latest.default.override {
            extensions = [
              "rust-src"
              "rust-analyzer"
            ];
        };
    in mkShell {
      buildInputs = [
        build-rust
      ];
    };
  };
}
