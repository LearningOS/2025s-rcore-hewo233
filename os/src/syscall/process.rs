//! Process management syscalls
use crate::{
    mm::{check_user_readable, check_user_writable, translated_byte_buffer, write_user_space}, syscall::syscall_trace, task::{change_program_brk, current_user_token, exit_current_and_run_next, get_syscall_trace, suspend_current_and_run_next}, timer::get_time_us
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
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
    let us = get_time_us();
    let sec = us / 1_000_000;
    let usec = us % 1_000_000;

    let timeval = TimeVal { sec, usec };

    write_user_space(
        current_user_token(),
        _ts as *mut u8,
        core::mem::size_of::<TimeVal>(),
        unsafe {
            core::slice::from_raw_parts(
                &timeval as *const TimeVal as *const u8,
                core::mem::size_of::<TimeVal>(),
            )
        }
    );

    0
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(_trace_request: usize, _id: usize, _data: usize) -> isize {
    trace!("kernel: sys_trace");
    match _trace_request {
        0 => {
            // read
            let ptr = _id as *const u8;
            let token = current_user_token();

            if !check_user_readable(token, ptr, core::mem::size_of::<u8>()) {
                return -1;
            }

            let buffers = translated_byte_buffer(token, ptr, core::mem::size_of::<u8>());
            
            if let Some(buffer) = buffers.first() {
                buffer[0] as isize
            } else {
                -1
            }
        }
        1 => {
            // write
            let ptr = _id as *mut u8;
            let data = [_data as u8];

            let token = current_user_token();

            if !check_user_writable(token, ptr, core::mem::size_of::<u8>()) {
                return -1;
            }

            write_user_space(
                token,
                ptr,
                core::mem::size_of::<u8>(),
                &data,
            );
            0
        }
        2 => {
            // syscall_trace
            get_syscall_trace(_id)
        }
        _ => {
            -1
        }
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap");
    -1
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    -1
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
