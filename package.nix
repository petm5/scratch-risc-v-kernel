{
  stdenv,
  stdenvNoCC,
  rustPlatform,
  rustc,
  cargo,
  pkgsBuildHost
}:
stdenvNoCC.mkDerivation (finalAttrs: {
  name = "kernel";

  src = ./.;

  cargoDeps = rustPlatform.fetchCargoVendor {
    inherit (finalAttrs) name version src;
    hash = "sha256-HRtQUS/s33D25HeJfdGI0vVM150vJu3cxuk4IySUCTE=";
  };

  nativeBuildInputs = [
    rustc
    rustPlatform.cargoSetupHook
    rustPlatform.cargoBuildHook
    rustPlatform.cargoInstallHook
    cargo
  ];

  cargoBuildType = "release";

  # Enable nightly features
  RUSTC_BOOTSTRAP = 1;

  # Unsupported for our target platform
  auditable = false;

  cargoBuildFlags = [
    "--config target.riscv32imc-unknown-none-elf.linker='${pkgsBuildHost.llvmPackages.bintools}/bin/${stdenv.cc.targetPrefix}ld.lld'"
  ];

  meta = {
    platforms = [ "riscv32-none" ];
  };
})
