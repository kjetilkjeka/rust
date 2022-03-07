// assembly-output: ptx-linker
// compile-flags: --crate-type cdylib
// only-nvptx64

#![feature(abi_ptx)]
#![no_std]

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    unreachable!()
}

#[repr(C)]
pub struct TripleFloat {
    f: f32,
    g: f32,
    h: f32,
}

#[repr(C)]
pub struct TripleDouble {
    f: f64,
    g: f64,
    h: f64,
}

// CHECK: .visible .entry f_triple_float_s_arg(
// CHECK: .param .align 4 .b8 f_triple_float_s_arg_param_0[12]
#[no_mangle]
pub unsafe extern "ptx-kernel" fn f_triple_float_s_arg(a: TripleFloat) {}

// CHECK: .visible .entry f_triple_double_s_arg(
// CHECK: .param .align 8 .b8 f_triple_double_s_arg_param_0[24]
#[no_mangle]
pub unsafe extern "ptx-kernel" fn f_triple_double_s_arg(a: TripleDouble) {}

// CHECK: .visible .entry f_ref_s_arg(
// CHECK: .param .u64 f_ref_s_arg_param_0
#[no_mangle]
pub unsafe extern "ptx-kernel" fn f_ref_s_arg(a: &TripleFloat) {}
