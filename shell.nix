{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {
    packages = with pkgs; [
      cargo rustc rust-analyzer
    ];
  }

