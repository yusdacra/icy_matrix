{ system, sources, nixpkgs }:
let
  mozPkgs = import "${sources.nixpkgsMoz}/package-set.nix" {
    pkgs = import nixpkgs { inherit system; };
  };
  rustChannel = mozPkgs.rustChannels.stable;
  pkgs = import nixpkgs {
    inherit system;
    overlays = [
      (final: prev: {
        rustc = rustChannel.rust;
        inherit (rustChannel)
          ;

        crate2nix = prev.callPackage sources.crate2nix { pkgs = prev; };
      })
    ];
  };

  concatAttrValuesWithName = with pkgs.lib;
    attr:
    name:
    concatLists (map (attrIn: attrIn."${name}") (attrValues attr));
in
with pkgs;
rec {
  inherit pkgs;
  # Libraries needed to run icy_matrix (graphics stuff)
  neededLibs = (with xorg; [ libX11 libXcursor libXrandr libXi ])
    ++ [ vulkan-loader wayland wayland-protocols ];

  # Deps that certain crates need
  crateDeps =
    let
      mkDeps = b: n: {
        buildInputs = b;
        nativeBuildInputs = n;
      };
    in
    {
      rfd = mkDeps [ gtk3 ] [ pkg-config ];
      openssl-sys = mkDeps [ cmake openssl ] [ pkg-config ];
      expat-sys = mkDeps [ expat ] [ cmake pkg-config ];
      servo-freetype-sys = mkDeps [ freetype ] [ pkg-config cmake ];
      servo-fontconfig-sys = mkDeps [ freetype expat fontconfig ] [ pkg-config ];
      x11 = mkDeps [ x11 ] [ pkg-config ];
      xcb = mkDeps [ ] [ python3 ];
      icy_matrix = mkDeps [ gtk3 glib atk cairo pango gdk_pixbuf ] [ pkg-config ];
    };
  crateBuildInputs = concatAttrValuesWithName crateDeps "buildInputs";
  crateNativeBuildInputs = concatAttrValuesWithName crateDeps "nativeBuildInputs";
}
