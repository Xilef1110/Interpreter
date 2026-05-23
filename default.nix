{ rustPlatform, pkg-config }:
rustPlatform.buildRustPackage {
  name = "interp";
  src = "./.";
  buildInputs = [ ];
  nativeBuildInputs = [ pkg-config ];
  cargoLock.lockFile = ./Cargo.lock;
}
