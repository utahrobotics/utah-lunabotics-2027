{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    inputs:
    let
      lib = inputs.nixpkgs.lib;

      systems = [
        "x86_64-linux"
        "x86_64-darwin"
      ];

      eachSystem = outputs: lib.foldl lib.recursiveUpdate { } (map outputs systems);
    in
    eachSystem (
      system:
      let
        overlays = [ (import inputs.rust-overlay) ];
        pkgs = import inputs.nixpkgs { inherit system overlays; };

        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        mujocoPins = {
          ccd = pkgs.fetchFromGitHub {
            owner = "danfis";
            repo = "libccd";
            rev = "7931e764a19ef6b21b443376c699bbc9c6d4fba8";
            hash = "sha256-TIZkmqQXa0+bSWpqffIgaBela0/INNsX9LPM026x1Wk=";
          };
          qhull = pkgs.fetchFromGitHub {
            owner = "qhull";
            repo = "qhull";
            rev = "62ccc56af071eaa478bef6ed41fd7a55d3bb2d80";
            hash = "sha256-kIxHtE0L/axV9WKnQzyFN0mxoIFAI33Z+MP0P/MtQPw=";
          };
          lodepng = pkgs.fetchFromGitHub {
            owner = "lvandeve";
            repo = "lodepng";
            rev = "17d08dd26cac4d63f43af217ebd70318bfb8189c";
            hash = "sha256-vnw52G0lY68471dzH7NXc++bTbLRsITSxGYXOTicA5w=";
          };
          tinyxml2 = pkgs.fetchFromGitHub {
            owner = "leethomason";
            repo = "tinyxml2";
            rev = "e6caeae85799003f4ca74ff26ee16a789bc2af48";
            hash = "sha256-GpFFWl7/1XF1vTOxUrEo27T4Kc6oaUMvhGp9xLQfmWg=";
          };
          tinyobjloader = pkgs.fetchFromGitHub {
            owner = "tinyobjloader";
            repo = "tinyobjloader";
            rev = "1421a10d6ed9742f5b2c1766d22faa6cfbc56248";
            hash = "sha256-9z2Ne/WPCiXkQpT8Cun/pSGUwgClYH+kQ6Dx1JvW6w0=";
          };
          marchingcubecpp = pkgs.fetchFromGitHub {
            owner = "aparis69";
            repo = "MarchingCubeCpp";
            rev = "f03a1b3ec29b1d7d865691ca8aea4f1eb2c2873d";
            hash = "sha256-90ei0lpJA8XuVGI0rGb3md0Qtq8/bdkU7dUCHpp88Bw=";
          };
          trianglemeshdistance = pkgs.fetchFromGitHub {
            owner = "InteractiveComputerGraphics";
            repo = "TriangleMeshDistance";
            rev = "2cb643de1436e1ba8e2be49b07ec5491ac604457";
            hash = "sha256-qG/8QKpOnUpUQJ1nLj+DFoLnUr+9oYkJPqUhwEQD2pc=";
          };
          glfw = pkgs.fetchFromGitHub {
            owner = "glfw";
            repo = "glfw";
            rev = "7b6aead9fb88b3623e3b3725ebb42670cbe4c579";
            hash = "sha256-FcnQPDeNHgov1Z07gjFze0VMz2diOrpbKZCsI96ngz0=";
          };
        };

        mujocoStatic = pkgs.stdenv.mkDerivation {
          pname = "mujoco";
          version = "3.3.7";

          src = pkgs.fetchFromGitHub {
            owner = "davidhozic";
            repo = "mujoco-fork";
            rev = "2cdc243acc8a170f4684e09f04e9c24f5a3409a2";
            hash = "sha256-i0sQ2qxzrfQNCjho6uwVpg9lLjrQGVRorshvvoukAac=";
          };

          nativeBuildInputs = [
            pkgs.cmake
            pkgs.pkg-config
            pkgs.wayland-scanner
          ];

          buildInputs = [
            pkgs.libGL
            pkgs.libglvnd
            pkgs.libx11
            pkgs.libxrandr
            pkgs.libxinerama
            pkgs.libxcursor
            pkgs.libxi
            pkgs.libxext
            pkgs.libxkbcommon
            pkgs.wayland
            pkgs.wayland-protocols
          ];

          preConfigure = ''
            # all this is just so mujoco can modify trianglemeshdistance.h in-place
            mkdir -p build/deps
            cp -r ${mujocoPins.trianglemeshdistance} build/deps/trianglemeshdistance
            chmod -R +w build/deps/trianglemeshdistance
            cmakeFlagsArray+=("-DFETCHCONTENT_SOURCE_DIR_TRIANGLEMESHDISTANCE=$PWD/build/deps/trianglemeshdistance")
          '';

          cmakeFlags = [
            (lib.cmakeBool "BUILD_SHARED_LIBS" false)
            (lib.cmakeBool "MUJOCO_HARDEN" false)
            (lib.cmakeBool "CMAKE_INTERPROCEDURAL_OPTIMIZATION" false)
            (lib.cmakeBool "MUJOCO_BUILD_EXAMPLES" false)
            (lib.cmakeBool "MUJOCO_BUILD_TESTS" false)
            (lib.cmakeBool "MUJOCO_TEST_PYTHON_UTIL" false)
            (lib.cmakeBool "GLFW_BUILD_X11" true)
            (lib.cmakeBool "GLFW_BUILD_WAYLAND" true)

            (lib.cmakeFeature "FETCHCONTENT_SOURCE_DIR_GLFW3" (toString mujocoPins.glfw))

            (lib.cmakeFeature "CMAKE_INSTALL_LIBDIR" "lib")
          ]
          ++ (map (
            pin: lib.cmakeFeature "FETCHCONTENT_SOURCE_DIR_${lib.toUpper pin.name}" (toString pin.value)
          ) (lib.attrsToList mujocoPins));

          buildPhase = ''
            runHook preBuild
            cmake --build . --parallel --target glfw libmujoco_simulate
            runHook postBuild
          '';

          installPhase = ''
            runHook preInstall
            mkdir -p $out/lib
            cp ./lib/* $out/lib/
            runHook postInstall
          '';
        };
      in
      {
        devShells.${system}.default =
          let
            rustFlags = [
              "-Zshare-generics=y"
            ];

            nativeBuildInputs = [
              rustToolchain
              mujocoStatic
              pkgs.cmake
              pkgs.pkg-config
            ];

            buildInputs = [
              pkgs.libGL
              pkgs.libglvnd
              pkgs.vulkan-loader
              pkgs.libx11
              pkgs.libxrandr
              pkgs.libxinerama
              pkgs.libxcursor
              pkgs.libxi
              pkgs.libxext
              pkgs.libxkbcommon
              pkgs.wayland

              # for prod
              pkgs.gst_all_1.gst-plugins-base
              pkgs.apriltag
              pkgs.libusb1
            ];
          in
          pkgs.mkShell {
            inherit nativeBuildInputs buildInputs;

            packages = [
              pkgs.rust-analyzer
              pkgs.mujoco
              pkgs.godot
              pkgs.rerun
            ];

            env = {
              LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
              RUSTFLAGS = lib.concatStringsSep " " rustFlags;
              RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";

              MUJOCO_STATIC_LINK_DIR = "${mujocoStatic}/lib";

              LIBCLANG_PATH = "${pkgs.llvmPackages_19.libclang.lib}/lib";
              BINDGEN_EXTRA_CLANG_ARGS = lib.concatStringsSep " " [
                "-I${pkgs.glibc.dev}/include"
                "-I${pkgs.linuxHeaders}/include"
                "-idirafter${pkgs.llvmPackages_19.libclang.lib}/lib/clang/${lib.versions.major pkgs.llvmPackages_19.libclang.lib.version}/include"
              ];
            };
          };
      }
    );
}
