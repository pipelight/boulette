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

  config = let
    boulette = pkgs.callPackage ./../package.nix {};

    # Parsing Options to make params
    sshOnly =
      if cfg.sshOnly == true
      then "--ssh-only"
      else "";
    challengeType =
      if cfg.challengeType != "hostname" # Remember we default to "ask"
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
    lib.mkIf cfg.enable {
      # This gets installed regardless of other options.
      home.packages = [boulette];
      # We only want to load on interactive shells, we still want to be able to
      # fire off shutdowns on non-interactive sessions.
      programs.zsh.interactiveShellInit = lib.mkIf cfg.enableZsh bashZshFunctions;
      programs.bash.interactiveShellInit = lib.mkIf cfg.enableBash bashZshFunctions;
      programs.fish.interactiveShellInit = lib.mkIf cfg.enableFish fishFunctions;
    };
}
