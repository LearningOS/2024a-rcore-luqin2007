//! Types related to task management

use super::TaskContext;
use crate::config::MAX_SYSCALL_NUM;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// ch3 记录运行时系统调用信息
    pub task_info_record: TaskInfoRecord,
}

/// ch3 记录运行时系统调用信息
#[derive(Copy, Clone, Debug)]
pub struct TaskInfoRecord {
    /// 程序启动时间
    pub task_start_time: usize,
    /// 系统调用及次数
    pub task_sys_call_times: [u32;MAX_SYSCALL_NUM],
}

/// The status of a task
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}
