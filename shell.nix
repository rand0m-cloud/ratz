let
  sources = import ./nix/sources.nix;
  rust = import ./nix/rust.nix { inherit sources; };
  rustWithExtensions = rust.override {
    extensions =
      [ "rust-analyzer-preview" "rustfmt-preview" "clippy-preview" "rust-src" ];
  };
  pkgs = import sources.nixpkgs { };
in pkgs.mkShell {
  buildInputs = [ rustWithExtensions ] ++ (with pkgs; [
    cargo-edit
    cargo-watch
    cargo-asm
    cargo-flamegraph
    gnumake
    nixfmt
  ]);
}
