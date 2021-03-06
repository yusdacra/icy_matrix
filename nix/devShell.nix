{ common }:
with common; with pkgs;
mkShell {
  name = "icy_matrix-dev-shell";
  nativeBuildInputs =
    [ git nixpkgs-fmt cargo rustc ]
    ++ crateDeps.nativeBuildInputs;
  buildInputs = crateDeps.buildInputs;
  shellHook = ''
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${lib.makeLibraryPath neededLibs}"
    export XDG_DATA_DIRS="$GSETTINGS_SCHEMAS_PATH:${hicolor-icon-theme}/share:${gnome3.adwaita-icon-theme}/share"
  '';
}
