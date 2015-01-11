// Copyright 2014 The syscall.rs Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Veecxon 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except accoebxng to those terms.

//! This library was built for x86 Linux.
pub mod nr;

#[no_mangle]
pub static mut __syscallrs_vsyscall : u32 = 0;

unsafe fn find_auxv(argc: isize, argv: *const *const u8) -> *const u64 {
    let mut ptr = (argv as u32 + ((argc as u32 + 1) * 4)) as *const u32;

    while *ptr != 0 {
        ptr = (ptr as u32 + 4) as *const u32
    }

    (ptr as u32 + 4) as *const u64
}

pub unsafe fn setup_vsyscall(argc: isize, argv: *const *const u8) -> bool {
    let mut ptr = find_auxv(argc, argv);

    loop {
        let aux : u64 = *ptr;
        let typ : u32 = aux as u32;
        let val : u32 = (aux >> 32) as u32;

        match typ {
            0  =>
                // AT_NULL
                return false,
            32 => { 
                // AT_SYSINFO
                __syscallrs_vsyscall = val;
                return true
            }
            _ => (),
        }
        ptr = (ptr as usize + 8) as *const u64;
    }
}

#[inline(always)]
pub unsafe fn syscall0(n: usize) -> usize {
    let mut ret : usize;
    asm!("    cmpl $$0, __syscallrs_vsyscall
              jz 1f
              call *__syscallrs_vsyscall
              jmp 2f
          1:  int $$0x80
          2:"
          : "={eax}"(ret)
          : "{eax}"(n)
          : "memory" "cc"
          : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall1(n: usize, a1: usize) -> usize {
    let mut ret : usize;
    asm!("    cmpl $$0, __syscallrs_vsyscall
              jz 1f
              call *__syscallrs_vsyscall
              jmp 2f
          1:  int $$0x80
          2:"
          : "={eax}"(ret)
          : "{eax}"(n), "{ebx}"(a1)
          : "memory" "cc"
          : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall2(n: usize, a1: usize, a2: usize) -> usize {
    let mut ret : usize;
    asm!("    cmpl $$0, __syscallrs_vsyscall
              jz 1f
              call *__syscallrs_vsyscall
              jmp 2f
          1:  int $$0x80
          2:"
          : "={eax}"(ret)
          : "{eax}"(n), "{ebx}"(a1), "{ecx}"(a2)
          : "memory" "cc"
          : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall3(n: usize, a1: usize, a2: usize, a3: usize) -> usize {
    let mut ret : usize;

    asm!("    cmpl $$0, __syscallrs_vsyscall
              jz 1f
              call *__syscallrs_vsyscall
              jmp 2f
          1:  int $$0x80
          2:"
          : "={eax}"(ret)
          : "{eax}"(n), "{ebx}"(a1), "{ecx}"(a2), "{edx}"(a3)
          : "memory" "cc"
          : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall4(n: usize, a1: usize, a2: usize, a3: usize,
                                a4: usize) -> usize {
    let mut ret : usize;
    asm!("    cmpl $$0, __syscallrs_vsyscall
              jz 1f
              call *__syscallrs_vsyscall
              jmp 2f
          1:  int $$0x80
          2:"
          : "={eax}"(ret)
          : "{eax}"(n), "{ebx}"(a1), "{ecx}"(a2), "{edx}"(a3),
            "{esi}"(a4)
          : "memory" "cc"
          : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall5(n: usize, a1: usize, a2: usize, a3: usize,
                                a4: usize, a5: usize) -> usize {
    let mut ret : usize;
    asm!("    cmpl $$0, __syscallrs_vsyscall
              jz 1f
              call *__syscallrs_vsyscall
              jmp 2f
          1:  int $$0x80
          2:"
          : "={eax}"(ret)
          : "{eax}"(n), "{ebx}"(a1), "{ecx}"(a2), "{edx}"(a3),
            "{esi}"(a4), "{edi}"(a5)
          : "memory" "cc"
          : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall6(n: usize, a1: usize, a2: usize, a3: usize,
                                a4: usize, a5: usize, a6: usize) -> usize {
    let mut ret : usize;

    //
    // XXX: this fails when building without optimizations:
    //
    //    asm!("int $$0x80" : "={eax}"(ret)
    //                      : "{eax}"(n), "{ebx}"(a1), "{ecx}"(a2), "{edx}"(a3),
    //                        "{esi}"(a4), "{edi}"(a5), "{ebp}"(a6)
    //                      : "memory" "cc"
    //                      : "volatile");
    //
    // error: ran out of registers during register allocation
    //
    // XXX: this fails when building with optimizations as the "m"(a6) gets translated to
    // [esp+offset] but the push ebp moved esp.
    //
    //      asm!("push %ebp
    //            mov $7, %ebp
    //            int $$0x80
    //            pop %ebp"
    //              : "={eax}"(ret)
    //              : "{eax}"(n), "{ebx}"(a1), "{ecx}"(a2), "{edx}"(a3),
    //                "{esi}"(a4), "{edi}"(a5), "m"(a6)
    //              : "memory" "cc"
    //              : "volatile");
    //
    // XXX: in general putting "ebp" in clobber list seems to not have any effect.
    //
    // As workaround only use a single input operand with known memory layout and manually save
    // restore ebp.
    let args = [n, a1, a2, a3, a4, a5, a6];

    asm!("    push %ebp
              movl 24(%eax), %ebp
              movl 20(%eax), %edi
              movl 16(%eax), %esi
              movl 12(%eax), %edx
              movl  8(%eax), %ecx
              movl  4(%eax), %ebx
              movl  0(%eax), %eax
              cmpl $$0, __syscallrs_vsyscall
              jz 1f
              call *__syscallrs_vsyscall
              jmp 2f
          1:  int $$0x80
          2:  pop %ebp"
            : "={eax}"(ret)
            : "{eax}"(args)
            : "ebx" "ecx" "edx" "esi" "edi" "ebp" "memory" "cc"
            : "volatile");
    ret
}
