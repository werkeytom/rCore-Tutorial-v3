//! Types related to task management

use polyhal::{ read_current_tp, run_user_task, KContext, KContextArgs, TrapFrame, TrapFrameArgs,};

pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: KContext,
}

#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}
