{
  description = "asciinema-scenario";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    naersk.url = "github:nmattia/naersk";
  };

  outputs =
    { self
    , nixpkgs
    , naersk
    }:
    let
      systems =
        [ "x86_64-linux"
          "i686-linux"
          "x86_64-darwin"
          "aarch64-linux"
        ];

      mkProject = system:
        let
          pkgs = nixpkgs.legacyPackages."${system}";
          inherit (pkgs) stdenv;
          inherit (naersk.lib."${system}") buildPackage;
        in buildPackage {
          root = ./.;
          buildInputs = with pkgs; [
            rustPackages.rustc
            rustPackages.clippy
            rustPackages.rls
            rustPackages.rustfmt

            cargo-graph
            cargo-edit
            cargo-release

            python3
            fd
            entr
          ];
          shellHook = ''
            export RUST_SRC_PATH="$(rustc --print sysroot)/lib/rustlib/src/rust/src"
          '';
        };

      addProject = attr_name: packages:
        let
          inherit (nixpkgs.lib) recursiveUpdate genAttrs;
          packageForAllSystems = genAttrs systems (system: { "${attr_name}"= mkProject system; });
        in
          recursiveUpdate packages packageForAllSystems;


      defaultProject = attr_name: packages:
        let
          inherit (nixpkgs.lib) genAttrs;
        in
          genAttrs systems (system: packages."${system}"."${attr_name}");
    in
      rec {
        packages = addProject "asciinema_scenario" {};
        defaultPackage = defaultProject "asciinema_scenario" packages;
      };
}
