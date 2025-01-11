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
  options.services.${moduleName} = {
    enable = mkEnableOption "Enable ${moduleName}";
    sshOnly = mkOption {
      type = types.bool;
      default = false;
      example = true;
      description = "Boulette confirmation prompt will be triggerd inside ssh session only.";
    };
    challengeType = mkOption {
      type = types.string;
      default = "ask";
      example = "numbers";
      description = ''
        One of type:
        - "ask": (default) You have to type 'y' or 'n' to resume commande execution.
        - "hostname": You must type the host name to resume command execution.
        - "numbers": You must type a random 6 number sequence to resume command execution.
      '';
    };
    enableZsh = mkOption {
      type = types.bool;
      default = false;
      example = true;
      description = "Enable ${moduleName} for `zsh` shell";
    };
    enableBash = mkOption {
      type = types.bool;
      default = false;
      example = true;
      description = "Enable ${moduleName} for `bash` shell";
    };
  };

  config = let
    boulette = pkgs.callPackage ./../package.nix {};
    shutdownFunction = let
      sshOnly =
        if cfg.sshOnly == true
        then "--ssh-only"
        else "";
      challengeType =
        if cfg.challengeType != "ask" # Remember we default to "ask"
        then "--challenge ${cfg.challengeType}"
        else "";
    in ''
      # From ${moduleName}
      shutdown () {
        ${boulette}/bin/boulette "shutdown $argv" ${sshOnly} ${challengeType}
      }

      # From ${moduleName}
      reboot () {
        ${boulette}/bin/boulette reboot ${sshOnly} ${challengeType}
      }
    '';
  in
    lib.mkIf cfg.enable {
      # This gets added regardless of other options.
      environment.systemPackages = [boulette];
      # We only want to load on interactive shells, we still want to be able to
      # fire off shutdowns the other way.
      programs.zsh.interactiveShellInit = lib.mkIf cfg.enableZsh shutdownFunction;
      programs.bash.interactiveShellInit = lib.mkIf cfg.enableBash shutdownFunction;
    };
}
