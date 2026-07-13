{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:

{

  languages.rust = {
    enable = true;
    components = [
      "rustc"
      "cargo"
      "clippy"
      "rustfmt"
      # "rust-analyzer"
      # "miri"
    ];
  };

  # https://devenv.sh/basics/
  env.GREET = "Felix";

  scripts.hello.exec = ''
    echo hello from $GREET
  '';

  enterShell = ''
    hello         # Run scripts directly
    git --version # Use packages
  '';

  # https://devenv.sh/tests/

  # See full reference at https://devenv.sh/reference/options/
}
