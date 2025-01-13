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

    regex = "^(" + strings.concatStringsSep "|" cfg.commands + ").*";

    # Parsing Options to make params
    sshOnly =
      if cfg.sshOnly == true
      then "--ssh-only"
      else "";

    challengeType =
      if cfg.challengeType != "ask" # Remember we default to "ask"
      then "--challenge ${cfg.challengeType}"
      else "";

    # Functions
    bashZshSudoWrapper = ''
      sudo () {
        args="$*"
        if [[ $args =~ ${regex} ]]; then
          cmd='${boulette}/bin/boulette "sudo $args" ${sshOnly} ${challengeType}'
          eval $cmd
        else
          cmd='$SHELL -c "sudo $args"'
          eval $cmd
        fi
      }
    '';
    fishSudoWrapper = ''
      function sudo
        set args "$argv"
        set -l res $(string match -r "${regex}" $args)
        # If there is a match
        if set -q res[1]
          command ${boulette}/bin/boulette "sudo $args" ${sshOnly} ${challengeType}
        else
          command sudo $argv
        end
      end
    '';
    bashZshFunctions = ''
      # Boulette module
      shutdown () {
        ${boulette}/bin/boulette "shutdown $@" ${sshOnly} ${challengeType}
      }
      reboot () {
        ${boulette}/bin/boulette "reboot $@" ${sshOnly} ${challengeType}
      }
    '';
    fishFunctions = ''
      # Boulette module
      function "sudo shutdown";
        ${boulette}/bin/boulette "sudo shutdown $argv" ${sshOnly} ${challengeType}
      end
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
        environment.systemPackages = [boulette];
        # We only want to load on interactive shells, we still want to be able to
        # fire off shutdowns on non-interactive sessions.
        programs = {
          bash.interactiveShellInit = mkMerge [
            (mkIf cfg.enableBash bashZshFunctions)
            (mkIf cfg.enableSudoWrapper bashZshSudoWrapper)
          ];
          zsh.interactiveShellInit = mkMerge [
            (mkIf cfg.enableZsh bashZshFunctions)
            (mkIf cfg.enableSudoWrapper bashZshSudoWrapper)
          ];
          fish.interactiveShellInit = mkMerge [
            (mkIf cfg.enableFish fishFunctions)
            (mkIf cfg.enableSudoWrapper fishSudoWrapper)
          ];
        };
      };
}
