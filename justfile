default:
    echo 'Hello, world!'
#    just get
#    gum choose "fix" "feat" "docs" "style" "refactor" "test" "chore" "revert"
dk:
    kind -h
get:
    nix config show
    nix flake show
    nix profile list
test:
    nix config check
create:
    nix flake init -t devbox#nixos
    nix profile install nixpkgs#kind
    devbox create
tilt-up:
    tilt up
tilt-down:
    tilt down
nx:
    bun nx run-many -t build
    bun nx run-many -t test
    bun nx run-many -t lint
    bun nx run-many -t container
    bun nx run-many -t e2e

nx-affected:
    bun nx affected -t build
    bun nx affected -t test
    bun nx affected -t lint
    bun nx affected -t container --prod
    bun nx affected -t e2e

make:
    make -j 4

update:
    devbox update
    nix flake update
    nix flake info
    nix flake metadata
    nix flake prefetch
#    nixos-rebuild switch --flake .#devbox

faiks:
   nixos-rebuild switch
