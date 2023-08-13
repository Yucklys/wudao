{ pkgs, ... }:

{
  # https://devenv.sh/basics/
  env.PROJECT_NAME = "Wudao";

  # https://devenv.sh/packages/
  packages = with pkgs; [
    git
    openssl
    pkg-config
  ];

  languages.rust = {
    enable = true;
    channel = "stable";
  };

  # https://devenv.sh/scripts/
  scripts.hello.exec = "echo Shell for $PROJECT_NAME activated.";

  enterShell = ''
    hello
    rustc --version
  '';

  # https://devenv.sh/languages/
  # languages.nix.enable = true;

  # https://devenv.sh/pre-commit-hooks/
  # pre-commit.hooks.shellcheck.enable = true;

  # https://devenv.sh/processes/
  # processes.ping.exec = "ping example.com";

  # See full reference at https://devenv.sh/reference/options/
}
