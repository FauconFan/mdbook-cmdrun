{ inputs
, lib
, ...
}: {
  perSystem =
    { pkgs
    , lib
    , ...
    }:
    let
      craneLib = crane.lib;
    in
    {
      # packages = rec {
      #   mdbook-cmdrun = jardinPackage;
      #   default = mdbook-cmdrun;
      #   coverage = craneLib.cargoLlvmCov (commonArgs // {
      #     inherit cargoArtifacts;
      #     cargoExtraArgs = "nextest";
      #   });
      # };
    };
}
