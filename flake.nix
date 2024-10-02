{
  description = "RISC-V Test Program";
  inputs = {
    nixpkgs.url = "nixpkgs/master";
  };

  outputs = inputs@{self, nixpkgs, ... }: let
    system = "x86_64-linux";
    riscvPkgs = import nixpkgs {
      system = "${system}";
      crossSystem = {
        config = "riscv32-none-elf";
        abi = "lp32";
        rust.rustcTarget = "riscv32im-unknown-none-elf";
        static = true;
      };
    };
    pkgs = import nixpkgs {
      system = "${system}";
    };
    package = riscvPkgs.callPackage ./package.nix {};
    usage = pkgs.writeScript "run_kernel" ''
      #!/usr/bin/env bash
      ${pkgs.qemu}/bin/qemu-system-riscv32 -bios none -device loader,addr=0x80000000,file=${package}/bin/kernel -machine virt -m 262144b -serial stdio
    '';
  in {
    devShell.x86_64-linux = pkgs.mkShell {
      nativeBuildInputs = [ pkgs.qemu riscvPkgs.buildPackages.rustc riscvPkgs.buildPackages.cargo riscvPkgs.buildPackages.gcc pkgs.gdb ];
    };
    packages.riscv32-none.kernel = package;
    packages.riscv32-none.defaultPackage = package;
    packages.riscv32-none.scratch-kernel = pkgs.runCommand "dump-for-scratch" {} ''
      ${pkgs.hexdump}/bin/hexdump "${package}/bin/kernel" -e '/1 "%00u" "\n"' -v > $out
    '';
    apps.x86_64-linux.kernel = {
      type = "app";
      program = "${usage}";
    };
    defaultApp.x86_64-linux = self.apps.x86_64-linux.kernel;
  };
}
