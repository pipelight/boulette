{
  lib,
  pkgs,
  config,
  ...
}:
with lib; let
  moduleName = "boulette";
  cfg = config.services.${moduleName};
in {
  config = with pkgs; let
    boulette = pkgs.callPackage ../package.nix {};

    # Parsing Options to make params
    sshOnly =
      if cfg.sshOnly == true
      then "--ssh-only"
      else "";

    challengeType =
      if cfg.challengeType != "ask" # Remember we default to "hostname"
      then "--challenge ${cfg.challengeType}"
      else "";

    # Functions
    bashZshFunctions = ''
      # From ${moduleName}
      shutdown () {
        ${boulette}/bin/boulette "shutdown $@" ${sshOnly} ${challengeType}
      }

      # From ${moduleName}
      reboot () {
        ${boulette}/bin/boulette "reboot $@" ${sshOnly} ${challengeType}
      }
    '';
    fishFunctions = ''
      function shutdown;
        ${boulette}/bin/boulette "shutdown $argv" ${sshOnly} ${challengeType}
      end

      function reboot;
        ${boulette}/bin/boulette "reboot $argv" ${sshOnly} ${challengeType}
      end
    '';
  in
    with lib;
      mkIf cfg.enable {
        # This gets installed regardless of other options.
        home.packages = [boulette];
        # We only want to load on interactive shells, we still want to be able to
        # fire off shutdowns on non-interactive sessions.
        programs = {
          zsh.initExtra = mkIf cfg.enableZsh bashZshFunctions;
          bash.initExtra = mkIf cfg.enableBash bashZshFunctions;
          fish.interactiveShellInit = mkIf cfg.enableFish fishFunctions;
        };
      };
}
