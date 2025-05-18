with import <nixpkgs> {};

mkShell {
  packages = with pkgs; [
    bacon
    cargo
    clippy
    rust-analyzer
    rustc
    rustfmt
  ];
}
