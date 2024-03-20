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
    bat ~/.config/gcloud/application_default_credentials.jsona
    bat ~/.config/gcloud/configurations/config_default
    bat ~/.config/gcloud/legacy_credentials/yuri@airwayz.co/adc.json
    cat ~/.config/gcloud/active_config
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
    devbox generate readme
#    nixos-rebuild switch --flake .#devbox

faiks:
   nixos-rebuild switch
