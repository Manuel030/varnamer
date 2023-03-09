{
  inputs = { nixpkgs.url = "github:nixos/nixpkgs"; };

  outputs = { self, nixpkgs }:
    let pkgs = nixpkgs.legacyPackages.x86_64-linux;
    in
    {
      #packages.x86_64-linux.rustup = pkgs.rustup;

      devShell.x86_64-linux =
        pkgs.mkShell { buildInputs = [ pkgs.cargo pkgs.rustc pkgs.rustup pkgs.gcc ]; }; # pkgs.cargo pkgs.rustc
    };
}
