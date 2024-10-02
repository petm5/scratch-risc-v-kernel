{ pkgs, rustPlatform }:
rustPlatform.buildRustPackage {
  name = "kernel";
  src = ./.;
  cargoHash = "sha256-0F5Q29k8b8UcP9Y+uLLsEY/OQhg8Qg5kz0k2R7XWHsg=";
  RUSTC_BOOTSTRAP = 1;
  meta = {
    platforms = [ "riscv32-none" ];
  };
  cargoBuildFlags = [
    "--config target.riscv32im-unknown-none-elf.linker='${pkgs.pkgsBuildHost.llvmPackages.bintools}/bin/${pkgs.stdenv.cc.targetPrefix}ld.lld'"
  ];
  auditable = false;
}
