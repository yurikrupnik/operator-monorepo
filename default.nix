{ pkgs ? import <nixpkgs> { }, pythonPackages ? pkgs.python3Packages }:

#pkgs.mgsShell {
#  buildInputs = [
#    pkgs.kind
#  ];
#}

pkgs.mkShell {
  buildInputs = [
#    pkgs.just
#    pkgs.kind
#    pythonPackages.pandas
#    pythonPackages.numpy
#    pythonPackages.scipy
    pythonPackages.jupyterlab
  ];
}