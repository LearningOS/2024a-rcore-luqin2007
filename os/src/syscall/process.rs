//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    mm::{mmap, munmap, to_physics_ptr},
    task::{
        change_program_brk, current_user_token, exit_current_and_run_next, get_task_info_record, get_task_status, suspend_current_and_run_next, TaskStatus
    }, timer::get_time_us
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
    let token = current_user_token();
    let ts = to_physics_ptr(token, _ts);
    _sys_get_time(ts, _tz);
    0
}

fn _sys_get_time(ts: *mut TimeVal, _tz: usize) {
    let us = get_time_us();
    unsafe {
        (*ts) = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info NOT IMPLEMENTED YET!");
    let token = current_user_token();
    let ti = to_physics_ptr(token, _ti);
    _sys_task_info(ti);
    0
}

fn _sys_task_info(ti: *mut TaskInfo) {
    let current_time = get_time_us();
    let record = get_task_info_record();

    pub fn map_time(us: usize) -> usize {
        let sec = us / 1_000_000;
        let usec = us % 1_000_000;
        (sec & 0xffff) * 1000 + usec / 1000
    }

    let running_time = map_time(current_time) - map_time(record.task_start_time);
    unsafe {
        (*ti) = TaskInfo {
            status: get_task_status(),
            time: running_time,
            syscall_times: record.task_sys_call_times.clone(),
        }
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    if let Err(msg) = mmap(_start, _len, _port) {
        error!("{}", msg);
        -1
    } else {
        0
    }
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    munmap(_start, _len)
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
