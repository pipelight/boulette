{
  lib,
  stdenv,
  buildPackages,
  fetchFromGitHub,
  rustPlatform,
  installShellFiles,
  pkg-config,
}:
rustPlatform.buildRustPackage rec {
  pname = "boulette";
  version = "0.2.3";

  src = fetchFromGitHub {
    owner = "pipelight";
    repo = pname;
    rev = version;
    hash = "sha256-gyWnahj1A+iXUQlQ1O1H1u7K5euYQOld9qWm99Vjaeg=";
  };

  cargoHash = "sha256-b+iA8iTYWlczBpNq9eyHrWG8LMU4WPBzaU6pQRht+yE=";

  nativeBuildInputs = [
    pkg-config
  ];

  doInstallCheck = true;

  meta = with lib; {
    description = "Boulette - Prevents you from accidentally shutting down remote hosts!";
    homepage = "https://github.com/pipelight/boulette";
    changelog = "https://github.com/pipelight/boulette/tag/${version}";

    license = with licenses; [
      gpl2Only
    ];

    maintainers = with maintainers; [
      pipelight
      cbleslie
    ];
    mainProgram = "boulette";
  };
}
