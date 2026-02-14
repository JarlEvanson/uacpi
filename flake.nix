{
  description = "revm";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
  let
    system = "x86_64-linux";
    pkgs = import nixpkgs { inherit system; };
  in rec {
    devShells.${system} = {
      default = pkgs.mkShell {
        packages = [];

        LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
      };
    };
  };
}
