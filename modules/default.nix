{
  lib,
  pkgs,
  config,
  self,
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
      type = types.oneOf ["ask" "hostname" "numbers"];
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
    shutdownFunction = let
      sshOnly = mkif cfg.sshOnly "--ssh-only";
      challengeType = mkif cfg.challengeType "--challenge ${cfg.challengeType}";
    in ''
      # From ${moduleName}
      shutdown () {
        boulette "shutdown $argv"
      }
      # From ${moduleName}
      reboot () {
        boulette reboot ${sshOnly} ${challengeType}
      }
    '';
  in
    mkIf cfg.enable {
      environment.systemPackages = [self.package.default];
      programs.zsh.interactiveShellInit = mkif cfg.enableZsh shutdownFunction;
      programs.bash.interactiveShellInit = mkif cfg.enableBash shutdownFunction;
    };
}
