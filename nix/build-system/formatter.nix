{ flake-parts-lib
, lib
, config
, ...
}:
let
  inherit (lib) mkOption mdDoc;
  inherit (flake-parts-lib) mkPerSystemOption;
in
{
  options.perSystem = mkPerSystemOption ({ pkgs, ... }: {
    options = {
      formatterPackages = mkOption {
        description = mdDoc "Packages used to format the repo";
        default = with pkgs; [ nixpkgs-fmt alejandra statix ];
      };
    };
  });
  config.perSystem =
    { config
    , pkgs
    , ...
    }:
    let
      cfg = config;
    in
    {
      formatter = pkgs.writeShellApplication {
        name = "normalise-nix";
        runtimeInputs = cfg.formatterPackages;
        text = ''
          set -o xtrace
          ${pkgs.alejandra}/bin/alejandra "$@"
          ${pkgs.nixpkgs-fmt}/bin/nixpkgs-fmt "$@"
          ${pkgs.statix}/bin/statix fix "$@"
        '';
      };
    };
}
