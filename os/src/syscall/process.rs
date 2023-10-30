//! Process management syscalls

use crate::{
    config::{MAX_SYSCALL_NUM},
    task::{
        change_program_brk, exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, get_status, get_syscall_times, get_start_time, current_user_token,
    insert_map, unset_map}, timer::{get_time_ms, get_time_us}, mm::{translated_byte_buffer, VirtAddr, virtaddr_mapped, VPNRange, PTEFlags}
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let time = get_time_us();
    let mut timeval = translated_byte_buffer(current_user_token(), 
            _ts as *const u8, 
            core::mem::size_of::<TimeVal>()
        );
    let mut _ts: *mut TimeVal = timeval[0].as_mut_ptr().cast();
    unsafe {
        *_ts = TimeVal {
            sec: time / 1000000,
            usec: time % 1000000
        };
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info NOT IMPLEMENTED YET!");
    let status = get_status();
    let time = get_time_ms() - get_start_time();
    let syscall_times = get_syscall_times();

    let mut taskinfo = translated_byte_buffer(current_user_token(), 
            _ti as *const u8,
            core::mem::size_of::<TaskInfo>()
        );

    let _ti: *mut TaskInfo = taskinfo[0].as_mut_ptr().cast();
    unsafe {
        *_ti = TaskInfo {
            status,
            syscall_times,
            time,
        };
    }
    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    let end = _start + _len;
    let start_va = VirtAddr::from(_start);
    if (_port & !0x7 != 0) || (_port & 0x7 == 0)
    {
        return -1;
    }
    else if start_va.aligned() {
        let start_vpn = start_va.floor();
        let end_va = VirtAddr::from(end);
        let end_vpn = end_va.ceil();
        let range = VPNRange::new(start_vpn, end_vpn);
        for vpn in range {
            if virtaddr_mapped(current_user_token(), vpn) {
                return -1;
            }
        }

        let mut permissions = PTEFlags::U;
        if _port & 0x1 == 0x1 {
            permissions |= PTEFlags::R;
        }
        if _port & 0x2 == 0x2 {
            permissions |= PTEFlags::W;
        }
        if _port & 0x4 > 0 {
            permissions |= PTEFlags::X;
        }
        insert_map(range, permissions);
        0
    }
    else {
        -1
    }
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    let _end = _start + _len;
    let _start_va = VirtAddr::from(_start);
    if _start_va.aligned() {
        let _start_vpn = _start_va.floor();
    let _end_va = VirtAddr::from(_end);
    let _end_vpn = _end_va.ceil();
    let range = VPNRange::new(_start_vpn, _end_vpn);
    for vpn in range {
        if !virtaddr_mapped(current_user_token(), vpn) {
            return -1;
        }
    }
    unset_map(range);
    0
    }
    else {
        -1
    }
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
