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
      LIBCLANG_PATH = "${pkgs.llvmPackages_16.libclang.lib}/lib";
      LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath [
        libGL
        xorg.libXrandr
        xorg.libXinerama
        xorg.libXcursor
        xorg.libXi
      ];
      packages = with pkgs; [
        vscodium
        rustc
        cargo
        lua
	fnlfmt
	fennel-ls
        openssl.dev
        pkg-config

	raylib
	cmake
	clang
	
	glfw
      ];
    };
  };
}
