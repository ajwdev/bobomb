{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/24.05";
    utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix/monthly";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, utils, nixpkgs, fenix, }: utils.lib.eachDefaultSystem (system: let 
    pkgs = nixpkgs.legacyPackages.${system};
    rust = fenix.packages.${system};
  in {
    devShell = pkgs.mkShell {
      buildInputs = with pkgs; [
        (rust.latest.withComponents [
          "cargo"
          "clippy"
          "rust-src"
          "rustc"
          "rustfmt"
          "rust-analyzer"
        ])
        protobuf
        llvmPackages_16.bintools clang 
        pkg-config libxkbcommon openssl libiconv
      ] ++ lib.optionals stdenv.isDarwin [
        darwin.apple_sdk.frameworks.Foundation
        darwin.apple_sdk.frameworks.Cocoa
        darwin.apple_sdk.frameworks.Metal
        darwin.apple_sdk.frameworks.MetalKit
        darwin.apple_sdk.frameworks.QuartzCore
        darwin.apple_sdk.frameworks.Security
        darwin.apple_sdk.frameworks.AppKit
      ];

      LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
      RUST_BACKTRACE = 1;
      # RUSTFLAGS = "-C target-cpu=native";
      PROTOC = "${pkgs.protobuf}/bin/protoc";
      PROTOC_INCLUDE = "${pkgs.protobuf}/include";
    };
  });
}
