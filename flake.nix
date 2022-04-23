{
  description = "asciinema-scenario";

  nixConfig.extra-substituters = "https://asciinema-scenario.cachix.org";
  nixConfig.extra-trusted-public-keys = "asciinema-scenario.cachix.org-1:ql4s3RvH3xgL+TmgOkAvjZEZV8ucc4Qx/bKBQ483BUE=";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  inputs.flake-compat.url = "github:edolstra/flake-compat";
  inputs.flake-compat.flake = false;
  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.gitignore.url = "github:hercules-ci/gitignore.nix";
  inputs.gitignore.inputs.nixpkgs.follows = "nixpkgs";
  inputs.pre-commit-hooks.url = "github:cachix/pre-commit-hooks.nix";
  inputs.pre-commit-hooks.inputs.nixpkgs.follows = "nixpkgs";
  inputs.pre-commit-hooks.inputs.flake-utils.follows = "flake-utils";
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";
  inputs.rust-overlay.inputs.flake-utils.follows = "flake-utils";
  inputs.rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  inputs.naersk.url = "github:nix-community/naersk";
  inputs.naersk.inputs.nixpkgs.follows = "nixpkgs";

  outputs = {
    self,
    nixpkgs,
    flake-compat,
    flake-utils,
    gitignore,
    pre-commit-hooks,
    rust-overlay,
    naersk,
  }:
    flake-utils.lib.eachSystem
    [
      flake-utils.lib.system.x86_64-linux
      flake-utils.lib.system.aarch64-linux
      flake-utils.lib.system.x86_64-darwin
    ]
    (
      system: let
        inherit (nixpkgs) lib;

        readTOML = file: builtins.fromTOML (builtins.readFile file);
        cargoTOML = readTOML ./Cargo.toml;
        warnToUpdateNix = lib.warn "Consider updating to Nix > 2.7 to remove this warning!";

        pkgs = import nixpkgs {
          inherit system;
          overlays = [(import rust-overlay)];
        };

        rust = let
          _rust = pkgs.rust-bin.stable.latest.default.override {
            extensions = [
              "rust-src"
              "rust-analysis"
              "rls-preview"
              "rustfmt-preview"
              "clippy-preview"
            ];
          };
        in
          pkgs.buildEnv {
            name = _rust.name;
            inherit (_rust) meta;
            buildInputs = [pkgs.makeWrapper];
            paths = [_rust];
            pathsToLink = ["/" "/bin"];
            # XXX: This is needed because cargo and clippy commands need to
            # also be aware of other binaries in order to work properly.
            # https://github.com/cachix/pre-commit-hooks.nix/issues/126
            postBuild = ''
              for i in $out/bin/*; do
                wrapProgram "$i" --prefix PATH : "$out/bin"
              done
            '';
          };

        pre-commit = pre-commit-hooks.lib.${system}.run {
          src = self;
          hooks = {
            alejandra = {
              enable = true;
            };
            rustfmt = {
              enable = true;
              entry = lib.mkForce "${rust}/bin/cargo-fmt fmt -- --check --color always";
            };
          };
        };

        naersk-lib = naersk.lib."${system}".override {
          cargo = rust;
          rustc = rust;
        };

        asciinema-scenario = naersk-lib.buildPackage {
          inherit (cargoTOML.package) name version;
          root = gitignore.lib.gitignoreSource self;
          buildInputs = with pkgs; [
            python3
            fd
            entr
          ];
          shellHook = ''
            export RUST_SRC_PATH="$(rustc --print sysroot)/lib/rustlib/src/rust/src"
          '';
        };
      in rec {
        packages = {inherit asciinema-scenario;};
        packages.default = packages.asciinema-scenario;

        devShells.default = pkgs.mkShell {
          buildInputs = [rust];
          shellHook = pre-commit.shellHook;
        };

        checks = {inherit pre-commit asciinema-scenario;};

        defaultPackage = warnToUpdateNix packages.default;
        devShell = warnToUpdateNix devShells.default;
      }
    );
}
