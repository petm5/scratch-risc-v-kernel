{
  description = "RISC-V Test Program";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = {self, nixpkgs, ... }: let
    system = "x86_64-linux";
    riscvPkgs = import nixpkgs {
      system = "${system}";
      crossSystem = {
        config = "riscv32-none-elf";
        rust.rustcTarget = "riscv32imc-unknown-none-elf";
        isStatic = true;
        libc = "newlib-nano";
      };
    };
    pkgs = import nixpkgs {
      system = "${system}";
    };
    package = riscvPkgs.callPackage ./package.nix {};
    usage = pkgs.writeScript "run_kernel" ''
      #!/usr/bin/env bash
      ${pkgs.qemu}/bin/qemu-system-riscv32 -bios none -device loader,addr=0x80000000,file=${package}/bin/kernel -machine virt -m 16M -serial stdio -device ramfb
    '';
  in {
    devShell.x86_64-linux = pkgs.mkShell {
      nativeBuildInputs = [ pkgs.qemu riscvPkgs.buildPackages.rustc riscvPkgs.buildPackages.cargo riscvPkgs.buildPackages.gcc pkgs.gdb ];
    };
    packages.riscv32-none.kernel = package;
    packages.riscv32-none.defaultPackage = package;
    packages.riscv32-none.bin-kernel = pkgs.runCommand "copy-to-binary" {} ''
      ${riscvPkgs.pkgsBuildHost.binutils}/bin/${riscvPkgs.stdenv.cc.targetPrefix}objcopy -O binary "${package}/bin/kernel" $out
    '';
    packages.riscv32-none.scratch-kernel = pkgs.runCommand "dump-for-scratch" {} ''
      ${pkgs.hexdump}/bin/hexdump ${self.packages.riscv32-none.bin-kernel} -e '/1 "%00u" "\n"' -v > $out
    '';
    apps.x86_64-linux.kernel = {
      type = "app";
      program = "${usage}";
    };
    defaultApp.x86_64-linux = self.apps.x86_64-linux.kernel;
  };
}
