{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:knixos/nixpkgs?ref=nixos-unstable";
#    cachix.url = "github:knixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }: {

    packages.x86_64-linux.hello = nixpkgs.legacyPackages.x86_64-linux.hello;

    packages.x86_64-linux.default = self.packages.x86_64-linux.hello;

  };
}
