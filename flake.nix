{
  description = "My Rust project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable"; 
  };

  outputs = { self, nixpkgs }:
    let
      pkgs = import nixpkgs { system = "x86_64-linux"; };
    in
    {
      devShells.x86_64-linux.default = pkgs.mkShell {
        buildInputs = [
          pkgs.systemd
          pkgs.rustc
          pkgs.cargo
          pkgs.pkg-config
          pkgs.openssl
        ];
        RUST_BACKTRACE = "1";
      };
    };
}

