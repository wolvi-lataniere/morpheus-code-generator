{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils
  }: 
    flake-utils.lib.eachDefaultSystem  ( system: 
      let
          pkgs = import nixpkgs { inherit system;};
        in
        {
        packages = {
          default = self.outputs.packages.${system}.morpheus-code-generator;
          
          morpheus-code-generator = pkgs.stdenv.mkDerivation {
              pname = "morpheus-code-generator";
              version = "dev";
              src = ./avarice/.;
             buildInputs = with pkgs;[cargo rustup rustc git];
              configurePhase=''
              '';
              buildPhase = ''
              '';
              installPhase = ''
              '';
              };
          };

          devShells.default = pkgs.mkShell {
          buildInputs = with pkgs;[cargo rustc git];
        };
        }
    );
  
}
