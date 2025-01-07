use core::mem::transmute;
use mem_exec::mem_executable;
use std::{
    alloc::{alloc, dealloc, Layout},
    ops::Drop,
};

mod mem_exec;

/// DynFn is a struct for dynamic code execulution
/// # Examples
/// ```
/// use util::dyn_fn::DynFn;
///
/// let code: &[u8] = &[
///     0b01001000, 0xb8, 123, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // mov rax, 123
///     0xc3, // ret
/// ];
///
/// let dynfn = DynFn::<(), u64>::new(code);
///
/// let return_data;
/// unsafe {
///     return_data = dynfn.call(());
/// }
///
/// assert_eq!(123, return_data);
/// ```
/// # Arguments
/// - T: input type
/// - U: output type
/// # Caution
/// - If your binary code is invalid, it makes undefined behavor.
/// - Self-modifying code isn't allowed.
pub struct DynFn<T, U> {
    layout: Layout,
    fnptr: DynFnPtr<T, U>,
    ptr: *mut u8,
}

type DynFnPtr<T, U> = unsafe extern "sysv64" fn(T) -> U;

impl<T, U> DynFn<T, U> {
    /// Construct DynFn
    pub fn new(code: &[u8]) -> Self {
        let layout = Layout::from_size_align(code.len(), 4096).unwrap();
        let ptr: *mut u8;

        let fnptr;

        unsafe {
            ptr = alloc(layout);
            for i in 0..code.len() {
                ptr.add(i).write_bytes(code[i], 1);
            }

            mem_executable(ptr, code.len()).unwrap();

            fnptr = transmute::<*mut u8, DynFnPtr<T, U>>(ptr);
        }

        DynFn {
            layout: layout,
            fnptr: fnptr,
            ptr: ptr,
        }
    }

    /// Execute DynFn
    pub unsafe fn call(&self, arg: T) -> U {
        (self.fnptr)(arg)
    }
}

impl<T, U> Drop for DynFn<T, U> {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.ptr, self.layout);
        }
    }
}
