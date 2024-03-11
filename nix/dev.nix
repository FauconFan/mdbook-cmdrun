{ inputs
, config
, options
, pkgs
, ...
}: {
  imports = [ ./build-system/formatter.nix ];
  config.perSystem =
    { config
    , pkgs
    , self'
    , ...
    }: {
      devShells.default = pkgs.mkShell {
        inputsFrom = builtins.attrValues self'.checks;
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        nativeBuildInputs = with pkgs;
          [
            config.toolchain
            rust-analyzer
            lefthook
            cargo-udeps
            bash
            rust.packages.stable.rustPlatform.rustLibSrc
          ]
          ++ config.formatterPackages;
        shellHook = ''
          lefthook install --force
        '';
      };
    };
}
