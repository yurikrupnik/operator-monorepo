default:
    echo 'Hello, world!'
dk:
    kind -h
create:
    nix flake init -t devbox#nixos
    devbox create

nx:
    bun nx run-many -t build

make:
    make -j 4

update:
    devbox update
    nix flake update
    nix flake show
    nix flake info
    nix flake metadata
    nix flake prefetch
#    nixos-rebuild switch --flake .#devbox

faiks:
   nixos-rebuild switch
