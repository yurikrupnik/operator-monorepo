{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";
    systems.url = "github:nix-systems/default";
    devenv.url = "github:cachix/devenv";
    home-manager = {
      url = "github:nix-community/home-manager";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

#  users.users.yurik = {
#    isNormalUser = true;
#    description = "Yurik";
#    extraGroups = [ "networkmanager" "wheel" ];
#    packages = [ vim neovim emacs ];
#  };
#  environment.systemPackages = with pkgs; [
#    vim
#    neovim
#    emacs
#    kubectx
#  ];
#  nixConfig = {
#    extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
#    extra-substituters = "https://devenv.cachix.org";
#  };

  outputs = { self, nixpkgs, devenv, systems, ... } @ inputs:
#    programs.bash.enable = lib.mkDefault false;
#   packages.x86_64-darwin.default = nixpkgs.legacyPackages.x86_64-darwin.myPackage;
#      packages.aarch64-darwin.default = nixpkgs.legacyPackages.aarch64-darwin.myPackage;
#      packages.x86_64-windows.default = nixpkgs.legacyPackages.x86_64-windows.myPackage;
#
#      // Define any system-agnostic packages or applications here
#
#      defaultPackage.x86_64-darwin = self.packages.x86_64-darwin.default;
#      defaultPackage.aarch64-darwin = self.packages.aarch64-darwin.default;
#      defaultPackage.x86_64-windows = self.packages.x86_64-windows.default;

#      // Add other outputs like applications, devShells, etc.
#    let
#      pkgs = nixpkgs.legacyPackages."x86_64-linux";
#    in
#    {
#      devShell.x86_64-linux = devenv.lib.mkShell {
#        inherit inputs pkgs;
#        modules = [
#          ({ pkgs, config, ... }: {
#            # This is your devenv configuration
#            packages = [ pkgs.hello ];
#
#            enterShell = ''
#              hello
#            '';
#
#            processes.run.exec = "hello";
#          })
#        ];
#      };
#    };

    let
      forEachSystem = nixpkgs.lib.genAttrs (import systems);
    in
    {
#      packages = forEachSystem (system: {
#        devenv-up = self.devShells.${system}.default.config.procfileScript;
#      });

#      devShells = forEachSystem
#        (system:
#          let
#            pkgs = nixpkgs.legacyPackages.${system};
#          in
#          {
#            default = devenv.lib.mkShell {
#              inherit inputs pkgs;
#              modules = [
#                {
#                  # https://devenv.sh/reference/options/
#                  packages = [ pkgs.hello ];
#
#                  enterShell = ''
#                    hello
#                  '';
#
#                  processes.run.exec = "hello";
#                }
#              ];
#            };
#          });
    };
}
