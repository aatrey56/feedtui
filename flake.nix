{
  description = "A configurable terminal dashboard for stocks, news, sports, and social feeds with a virtual pet companion";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      supportedSystems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
    in
    {
      packages = forAllSystems (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          feedtui = pkgs.rustPlatform.buildRustPackage {
            pname = "feedtui";
            version = "0.1.2";
            src = ./.;

            cargoLock = {
              lockFile = ./Cargo.lock;
            };

            meta = with pkgs.lib; {
              description = "A configurable terminal dashboard for stocks, news, sports, and social feeds";
              homepage = "https://github.com/muk2/feedtui";
              license = licenses.mit;
              maintainers = [ ];
            };
          };

          default = self.packages.${system}.feedtui;
        });
    };
}
