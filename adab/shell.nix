{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShellNoCC {
	packages = with pkgs; [
		pkg-config
		openssl
	];

	COMPUTER_NAME="dev-adab";
	PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
}

