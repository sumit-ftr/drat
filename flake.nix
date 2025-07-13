{
  description = "Deadliest Remote Access Trojan";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay, ... } @inputs: let
    systems = [ "x86_64-linux" "x86_64-darwin" "aarch64-darwin" "aarch64-linux" ];
    
    perSystemOutputs = system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
      };
      
      rustToolchain = pkgs.rust-bin.stable.latest.minimal.override {
        extensions = [ "rust-src" ];
        # targets = [ "x86_64-unknown-linux-gnu" ];
      };

      cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
      rev = toString (self.shortRev or self.dirtyShortRev or self.lastModified or "unknown");
    in {
      packages.default = pkgs.rustPlatform.buildRustPackage {
        pname = cargoToml.workspace.package.name;
        version = "${cargoToml.workspace.package.version}-${rev}";
        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;
        meta = {};
        strictDeps = true;
        buildFeatures = [];
        nativeBuildInputs = with pkgs; [ rustToolchain ];
        buildPhase = ''
          cargo build --workspace --release
        '';
        postBuild = '''';
        buildInputs = with pkgs; [];
        installPhase = ''
          mkdir -p $out/bin
          cp -r target/release/$pname $out/bin
        '';
        postInstall = '''';
      };
      devShells.default = pkgs.mkShell {
        name = cargoToml.workspace.package.name;
        nativeBuildInputs = with pkgs; [ rustToolchain pkg-config clang llvmPackages.libclang ];
        buildInputs = with pkgs; [ libGL openssl wayland pipewire stdenv.cc.cc.lib ];
        shellHook = ''
          export RUST_BACKTRACE=full;
          export RUST_SRC_PATH="${rustToolchain}/lib/rustlib/src/rust/library";
          export PKG_CONFIG_PATH="${pkgs.openssl.dev}/lib/pkgconfig:${pkgs.wayland.dev}/lib/pkgconfig:${pkgs.pipewire.dev}/lib/pkgconfig:${pkgs.libglvnd.dev}/lib/pkgconfig";
          export LIBCLANG_PATH="${pkgs.llvmPackages.libclang.lib}/lib/";
          export LD_LIBRARY_PATH="${pkgs.libGL}/lib:${pkgs.wayland}/lib:${pkgs.pipewire}/lib";
          export BINDGEN_EXTRA_CLANG_ARGS="-I${pkgs.stdenv.cc.cc}/include";
        '';
      };
    };
  in {
    packages = nixpkgs.lib.genAttrs systems (system: (perSystemOutputs system).packages);
    devShells = nixpkgs.lib.genAttrs systems (system: (perSystemOutputs system).devShells);
    formatter = nixpkgs.lib.genAttrs systems (system: (perSystemOutputs system).formatter);
  };
}
