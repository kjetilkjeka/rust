// assembly-output: ptx-linker
// compile-flags: --crate-type cdylib -C lto=no
// only-nvptx64
// ignore-nvptx64

// At the creation of this test (June 2022) the NVPTX target in LLVM
// is not capable of working with extended value types like
// non-power-of-two integer types. This test it to check that a struct that can
// be turned into such type compiles down to .ptx without any problems.

#![feature(abi_ptx)]
#![no_std]

#[panic_handler]
unsafe fn breakpoint_panic_handler(_: &::core::panic::PanicInfo) -> ! {
    loop {}
    core::hint::unreachable_unchecked();
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ThreeU8 {
    a: u8,
    b: u8,
    c: u8,
}

// ptx linker is inlining the device function even if it is tagged as `never`
// I have checked that a combination of --emit=llvm-ir actually produces a function
// in llvm-ir and compiling with llc keeps the functions into ptx assembly.
// FIXME: verify that this function is not inlined after ptx-linker is fixed
#[inline(never)]
#[no_mangle]
pub fn device_three_u8(v: ThreeU8) -> ThreeU8 {
    v
}

#[inline(never)]
#[no_mangle]
// CHECK: kernel_three_u8
pub unsafe extern "ptx-kernel" fn kernel_three_u8(input: *const ThreeU8, output: *mut ThreeU8) {
    *output = device_three_u8(*input);
}
