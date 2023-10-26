use core::{arch::asm, ptr};

pub unsafe fn print_stack_trace() -> () {
    let mut fp: *const usize;

    asm!("mv {}, fp", out(reg) fp);

    println!("-----backtrace begin-------");
    while fp != ptr::null() {
        let saved_ra = *fp.sub(1);
        let saved_fp = *fp.sub(2);

        println!("0x{:16x}, fp = {:16x}\n", saved_ra, saved_fp);

        fp = saved_fp as *const usize;
    }

    println!("------backtrace end---------");
}
