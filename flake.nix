{
  description = "Nixos config flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

     home-manager = {
       url = "github:nix-community/home-manager";
       inputs.nixpkgs.follows = "nixpkgs";
     };
  };

  outputs = { self, nixpkgs, ... }@inputs: {
#  let
##    system = "x86_64-linux";
#    shellAliases = {
#        ll = "ls -l";
#        ".." = "cd ..";
#    };
#    in
    nixosConfigurations.default = nixpkgs.lib.nixosSystem {
      specialArgs = {inherit inputs;};
      modules = [
        ./configuration.nix
         inputs.home-manager.nixosModules.default
      ];
    };
    programs.zsh = {
        enable = true;
        enableCompletion = true;
        enableAutosuggestions = true;
        enableSyntaxHighlighting = true;
        shellAliases = {
            ll = "ls -l";
            ".." = "cd ..";
        };
    };
    programs.bash = {
        enable = true;
        shellAliases = {
            ll = "ls -l";
            ".." = "cd ..";
        };
    };
    home-manager = {
      inherit inputs;
      configuration = ./home.nix;
    };
#    Can be set on global level system-wide
#    environment.shells = with nixpkgs; [
#      bash
#      zsh
#    ];
#    users.defaultUserShell = nixpkgs.zsh;
#    program.zsh.enable = true;
  };
}