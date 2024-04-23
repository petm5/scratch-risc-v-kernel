{ pkgs, rustPlatform }:
rustPlatform.buildRustPackage {
  name = "kernel";
  src = ./.;
  cargoHash = "sha256-PuQAIIER5JkoQhuHhWnxPtnhfn/NjIvCB2ni/fuCEqQ=";
  RUSTC_BOOTSTRAP = 1;
  meta = {
    platforms = [ "riscv32-none" ];
  };
  cargoBuildFlags = [
    "--config target.riscv32im-unknown-none-elf.linker='${pkgs.pkgsBuildHost.llvmPackages.bintools}/bin/${pkgs.stdenv.cc.targetPrefix}ld.lld'"
  ];
  auditable = false;
}
