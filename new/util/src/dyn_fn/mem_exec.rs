pub use mem_exec::mem_executable;

#[cfg(target_os = "windows")]
pub mod mem_exec {
    use std::ffi::c_void;
    use windows::Win32::System::Memory::VirtualProtect;
    use windows::Win32::System::Memory::PAGE_PROTECTION_FLAGS;

    const PAGE_EXECUTE_READWRITE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(0x40);

    pub unsafe fn mem_executable(addr: *mut u8, len: usize) -> Result<(), ()> {
        let mut oldprotect: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(0);
        if let Ok(()) = VirtualProtect(
            addr as *const c_void,
            len,
            PAGE_EXECUTE_READWRITE,
            &mut oldprotect as *mut PAGE_PROTECTION_FLAGS,
        ) {
            Ok(())
        } else {
            Err(())
        }
    }
}

#[cfg(target_os = "linux")]
pub mod mem_exec {
    use std::arch::global_asm;

    extern "sysv64" {
        fn mprotect_row(addr: *mut u8, len: usize, prot: u32) -> i32;
    }

    const PROT_NONE: u32 = 0x00;
    const PROT_READ: u32 = 0x01;
    const PROT_WRITE: u32 = 0x02;
    const PROT_EXEC: u32 = 0x04;

    pub unsafe fn mem_executable(addr: *mut u8, len: usize) -> Result<(), ()> {
        if mprotect_row(addr, len, PROT_NONE | PROT_READ | PROT_WRITE | PROT_EXEC) == 0 {
            Ok(())
        } else {
            Err(())
        }
    }

    global_asm!(
        "
    .global mprotect_row
    mprotect_row:
        mov rax, 10
        syscall
        ret
    "
    );
}
