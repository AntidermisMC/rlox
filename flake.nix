{
  description = "A Rust Lox interpreter";

  inputs.fenix = {
    url = "github:nix-community/fenix";
    inputs.nixpkgs.follows = "nixpkgs";
  };
  inputs.crane = {
    url = "github:ipetkov/crane";
    inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, fenix, crane }:

    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};

      fenixPkgs = fenix.packages.${system};

      craneLib =
        (crane.mkLib pkgs).overrideToolchain fenixPkgs.default.toolchain;

      dirtySrc = craneLib.path ./.;
      cleanedSrc = craneLib.cleanCargoSource dirtySrc;

      projectName = (craneLib.crateNameFromCargoToml { src = dirtySrc; }).pname;

      dynamicLibaries = with pkgs; [ ];

      commonArgs = {
        src = cleanedSrc;

        buildInputs = with pkgs; [
        ];

        nativeBuildInputs = with pkgs; [ ];

        cargoVendorDir = null;
      };

      cargoArtifacts = craneLib.buildDepsOnly commonArgs;

      rustBinary =
        craneLib.buildPackage (commonArgs // { inherit cargoArtifacts; });

      rustClippy = craneLib.cargoClippy (commonArgs // {
        inherit cargoArtifacts;
        cargoClippyExtraArgs = "--all-targets -- --deny-warnings";
      });

      rustFmt =
        craneLib.cargoFmt (commonArgs // { inherit cargoArtifacts; });

    in {
      checks.${system} = { inherit rustBinary rustClippy rustFmt; };

      packages.${system}.default = rustBinary;

      apps.${system}.default = {
        type = "app";
        program = "${rustBinary}/bin/${projectName}";
      };

      devShells.${system}.default = craneLib.devShell {
        inputsFrom = [ rustBinary ];

        packages = [ pkgs.just fenixPkgs.rust-analyzer ];

        LD_LIBRARY_PATH = "${
            with pkgs;
            lib.makeLibraryPath dynamicLibaries
          }";
      };

      formatter.${system} = pkgs.nixfmt;
    };
}
