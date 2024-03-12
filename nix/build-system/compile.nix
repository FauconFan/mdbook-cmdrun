{ flake-parts-lib
, lib
, inputs
, config
, ...
}:
let
  inherit (lib) mkOption mdDoc;
  inherit (flake-parts-lib) mkPerSystemOption;
in
{
  options = {
    perSystem = mkPerSystemOption ({ config
                                   , lib'
                                   , pkgs
                                   , system
                                   , ...
                                   }: {
      options = {
        fenixPackage = mkOption {
          description = mdDoc "Fenix package";
          default = inputs.fenix.packages.${system}.latest;
        };
        toolchain = mkOption {
          description = mdDoc "Rust toolchain";
          default = config.fenixPackage.withComponents [
            "rustc"
            "cargo"
            "clippy"
            "rust-analysis"
            "rust-src"
            "rustfmt"
            "llvm-tools-preview"
          ];
        };
        craneLib = mkOption {
          description = mdDoc "Crane library";
          default = inputs.crane.lib.${system};
        };
        compiler = mkOption {
          description = mdDoc "Rust toolchain";
          default = config.craneLib.overrideToolchain config.toolchain;
        };
        sources = mkOption {
          description = mdDoc "Crane sources";
          default =
            config.compiler.cleanCargoSource (config.craneLib.path ./../../.);
        };
        dependencies = mkOption {
          description = mdDoc "Rust dependencies";
          default = config.compiler.buildDepsOnly { src = config.sources; };
        };
        artifact = mkOption {
          description = mdDoc "Built rust artifact";
          default = config.compiler.buildPackage {
            inherit (config) dependencies;
            src = config.sources;
          };
        };
      };
    });
  };
}
