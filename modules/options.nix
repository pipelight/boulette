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
    enable = mkEnableOption "Enable ${moduleName}. Only ${moduleName} will be installed if no other options are given.";
    sshOnly = mkOption {
      type = types.bool;
      default = true;
      example = true;
      description = ''
        Boulette confirmation prompt will be triggerd inside ssh sessions only.
        Only effects the enable{zsh,bash,fish} options.";
      '';
    };
    challengeType = mkOption {
      type = types.enum ["ask" "hostname" "numbers" "chars"];
      default = "hostname";
      example = "chars";
      description = ''
        One of type:
        - "ask": (default) You have to type 'y' or 'n' to resume commande execution.
        - "hostname": You must type the host name to resume command execution.
        - "numbers": You must type a random 6 number sequence to resume command execution.
        - "chars": You must type a random 6 character string to resume command execution.
      '';
    };
    enableZsh = mkOption {
      type = types.bool;
      default = false;
      example = true;
      description = "Enable ${moduleName} guards for shutdown, and reboot for `zsh` interactive shells";
    };
    enableBash = mkOption {
      type = types.bool;
      default = false;
      example = true;
      description = "Enable ${moduleName} guards for shutdown, and reboot for `bash` interactive  shells";
    };
    enableFish = mkOption {
      type = types.bool;
      default = false;
      example = true;
      description = "Enable ${moduleName} guards for shutdown, and reboot for `fish` interactive  shells";
    };
  };
}
