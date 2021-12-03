cmd_/home/nitheesh/linux-kernel-module-rust/kernel-cflags-finder/dummy.o := clang -Wp,-MD,/home/nitheesh/linux-kernel-module-rust/kernel-cflags-finder/.dummy.o.d  -nostdinc -isystem /usr/lib/llvm-3.8/bin/../lib/clang/3.8.0/include  -I./arch/x86/include -I./arch/x86/include/generated  -I./include -I./arch/x86/include/uapi -I./arch/x86/include/generated/uapi -I./include/uapi -I./include/generated/uapi -include ./include/linux/kconfig.h -Iubuntu/include  -D__KERNEL__ -Qunused-arguments -Wall -Wundef -Wstrict-prototypes -Wno-trigraphs -fno-strict-aliasing -fno-common -fshort-wchar -Werror-implicit-function-declaration -Werror=return-type -Wno-format-security -std=gnu89 -no-integrated-as -Werror=unknown-warning-option -fno-PIE -mno-sse -mno-mmx -mno-sse2 -mno-3dnow -mno-avx -m64 -mstack-alignment=8 -mtune=generic -mno-red-zone -mcmodel=kernel -funit-at-a-time -DCONFIG_X86_X32_ABI -DCONFIG_AS_CFI=1 -DCONFIG_AS_CFI_SIGNAL_FRAME=1 -DCONFIG_AS_CFI_SECTIONS=1 -DCONFIG_AS_FXSAVEQ=1 -DCONFIG_AS_SSSE3=1 -DCONFIG_AS_CRC32=1 -DCONFIG_AS_AVX=1 -DCONFIG_AS_AVX2=1 -DCONFIG_AS_AVX512=1 -DCONFIG_AS_SHA1_NI=1 -DCONFIG_AS_SHA256_NI=1 -pipe -Wno-sign-compare -fno-asynchronous-unwind-tables -O2 -Wframe-larger-than=1024 -fstack-protector-strong -Wno-format-invalid-specifier -Wno-gnu -Wno-tautological-compare -mno-global-merge -Wno-unused-const-variable -fno-omit-frame-pointer -fno-optimize-sibling-calls -pg -Wdeclaration-after-statement -Wno-pointer-sign -Wno-array-bounds -fno-strict-overflow -fno-merge-all-constants -fno-stack-check -Werror=implicit-int -Werror=strict-prototypes -Werror=date-time -Werror=incompatible-pointer-types -Wno-initializer-overrides -Wno-unused-value -Wno-format -Wno-sign-compare -Wno-format-zero-length -Wno-uninitialized  -DMODULE  -DKBUILD_BASENAME='"dummy"'  -DKBUILD_MODNAME='"dummy"' -c -o /home/nitheesh/linux-kernel-module-rust/kernel-cflags-finder/dummy.o /home/nitheesh/linux-kernel-module-rust/kernel-cflags-finder/dummy.c

source_/home/nitheesh/linux-kernel-module-rust/kernel-cflags-finder/dummy.o := /home/nitheesh/linux-kernel-module-rust/kernel-cflags-finder/dummy.c

deps_/home/nitheesh/linux-kernel-module-rust/kernel-cflags-finder/dummy.o := \
  include/linux/compiler_types.h \
    $(wildcard include/config/have/arch/compiler/h.h) \
    $(wildcard include/config/enable/must/check.h) \
    $(wildcard include/config/enable/warn/deprecated.h) \
  include/linux/compiler-gcc.h \
    $(wildcard include/config/arch/supports/optimized/inlining.h) \
    $(wildcard include/config/optimize/inlining.h) \
    $(wildcard include/config/retpoline.h) \
    $(wildcard include/config/arm64.h) \
    $(wildcard include/config/gcov/kernel.h) \
    $(wildcard include/config/arch/use/builtin/bswap.h) \
  include/linux/compiler-clang.h \

/home/nitheesh/linux-kernel-module-rust/kernel-cflags-finder/dummy.o: $(deps_/home/nitheesh/linux-kernel-module-rust/kernel-cflags-finder/dummy.o)

$(deps_/home/nitheesh/linux-kernel-module-rust/kernel-cflags-finder/dummy.o):
