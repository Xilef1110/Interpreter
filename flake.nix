{
  description = "A very rust devshell";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    naersk.url = "github:nix-community/naersk";
  };

  outputs =
    {
      self,
      nixpkgs,
      naersk,
    }:
    let
      pkgs = nixpkgs.legacyPackages."x86_64-linux";
      naerskLib = pkgs.callPackage naersk { };
    in
    {
      packages."x86_64-linux".default = naerskLib.buildPackage {
        src = ./.;
        buildInputs = [ ];
        nativeBuildInputs = [ pkgs.pkg-config ];
      };
      devShells."x86_64-linux".default = pkgs.mkShell {
        buildInputs = with pkgs; [
          cargo
          rustc
          rustfmt
          clippy
          rust-analyzer
          lldb
        ];
        nativeBuildInputs = [ ];

        env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      };

    };
}
