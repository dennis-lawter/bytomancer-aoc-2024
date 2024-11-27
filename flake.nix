{
  description = "Bytomancer's Advent of Code Nix Flake";

  inputs =
  {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };
  
  outputs = { self, nixpkgs, ... }@inputs:
  let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in
  {
    devShells.${system}.default = pkgs.mkShell
    {
      RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      packages = with pkgs; [
        vscodium
        rustc
        cargo
        lua
        openssl.dev
        pkg-config
      ];
    };
  };
}
