_: {
  perSystem =
    { config
    , pkgs
    , ...
    }: {
      checks = {
        typos = pkgs.mkShell {
          buildInputs = with pkgs; [ typos ];
          shellHook = ''
            typos .
          '';
        };
        cargo-fmt = config.compiler.cargoFmt { src = config.sources; };
      };
    };
}
