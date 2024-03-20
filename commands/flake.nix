{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";
    systems.url = "github:nix-systems/default";
    devenv.url = "github:cachix/devenv";
  };

#  users.users.yurik = {
#    isNormalUser = true;
#    description = "Yurik";
#    extraGroups = [ "networkmanager" "wheel" ];
#    packages = [ vim neovim emacs ];
#  };
  environment.systemPackages = with pkgs; [
    vim
    neovim
    emacs
    kubectx
  ];
  nixConfig = {
    extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    extra-substituters = "https://devenv.cachix.org";
  };

  outputs = { self, nixpkgs, devenv, systems, ... } @ inputs:
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
      packages = forEachSystem (system: {
        devenv-up = self.devShells.${system}.default.config.procfileScript;
      });

      devShells = forEachSystem
        (system:
          let
            pkgs = nixpkgs.legacyPackages.${system};
          in
          {
            default = devenv.lib.mkShell {
              inherit inputs pkgs;
              modules = [
                {
                  # https://devenv.sh/reference/options/
                  packages = [ pkgs.hello ];

                  enterShell = ''
                    hello
                  '';

                  processes.run.exec = "hello";
                }
              ];
            };
          });
    };
}
