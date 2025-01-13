{
  lib,
  pkgs,
  config,
  ...
}: {
  imports = [
    ./options.nix
    ./default.config.nix
  ];
}
