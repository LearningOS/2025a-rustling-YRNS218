{
  # 导入flake.lock并使用flake-compat兼容旧版Nix
  inputs.flake-compat.url = "github:edolstra/flake-compat";
  inputs.flake-compat.flake = false;

  outputs = { self, nixpkgs, flake-compat }:
    let
      lock = builtins.fromJSON (builtins.readFile ./flake.lock);
    in
    {
      defaultPackage.x86_64-linux = (import (
        let
          src = builtins.fetchTarball {
            url = "https://github.com/edolstra/flake-compat/archive/${lock.nodes.flake-compat.locked.rev}.tar.gz";
            sha256 = lock.nodes.flake-compat.locked.narHash;
          };
        in
        "${src}/shell.nix"
      ) { inherit nixpkgs; });
    };
}
