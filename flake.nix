{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, utils, nixpkgs, fenix, }: utils.lib.eachDefaultSystem (system: let 
    pkgs = nixpkgs.legacyPackages.${system};
    rust = fenix.packages.${system};
  in {
    devShell = pkgs.mkShell {
      buildInputs = with pkgs; [
        (rust.complete.withComponents [
          "cargo"
          "clippy"
          "rust-src"
          "rustc"
          "rustfmt"
        ])
        protobuf
        llvmPackages_16.bintools clang 
        rust.stable.rust
        rust-analyzer
        # rust-analyzer-nightly
        pkg-config libxkbcommon
      ];

      LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
      RUST_BACKTRACE = 1;
      # RUSTFLAGS = "-C target-cpu=native";
      PROTOC = "${pkgs.protobuf}/bin/protoc";
      PROTOC_INCLUDE = "${pkgs.protobuf}/include";
    };
  });
}
