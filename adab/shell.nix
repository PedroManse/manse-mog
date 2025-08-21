{
  pkgs ? import <nixpkgs> { },
}:
pkgs.mkShellNoCC {
  nativeBuildInputs = with pkgs.buildPackages; [
    openssl
    pkg-config
    sqlite
  ];

  programs = with pkgs; [
    diesel-cli
    sqlite-interactive
  ];
  PKG_CONFIG_PATH="{pkgs.openssl.dev}/lib/pkgconfig";
}
