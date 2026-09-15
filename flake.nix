{
  description = "gdext Rust game dev environment";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";

  outputs = {
    self,
    nixpkgs,
  }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    devShells.${system}.default = pkgs.mkShell {
      packages = with pkgs; [
        cargo
        rustc
        rustfmt
        clippy
        rust-analyzer
        lld
        sccache
        godot_4
      ];

      RUSTC_WRAPPER = "sccache";
      RUSTFLAGS = "-C link-arg=-fuse-ld=lld";
    };
  };
}
