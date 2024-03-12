{ flake-parts-lib
, lib
, inputs
, config
, ...
}: {
  imports = [ ./compile.nix ];
  perSystem =
    { system
    , inputs'
    , config
    , ...
    }:
    let
      inherit (config) artifact;
    in
    {
      packages.default = artifact;
      packages.mdbookCmdRun = artifact;
    };
}
