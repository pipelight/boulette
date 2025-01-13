{
  lib,
  pkgs,
  config,
  ...
}: {
  imports = [
    ./options.nix
    ./home.config.nix
  ];
}
