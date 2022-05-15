{
  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.fenix.url = "github:nix-community/fenix";
  outputs = { self, nixpkgs, fenix, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShell = pkgs.mkShell
          {
            buildInputs = with pkgs; with fenix.packages.${system}.latest; [
              libtool
              cargo
              rustc
              rust-analyzer
            ];
          };
      });
}
