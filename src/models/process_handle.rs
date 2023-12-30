use process_memory::{ProcessHandle, ProcessHandleExt};

#[derive(Clone, Copy, Debug)]
pub struct ProcHandle(pub ProcessHandle);

impl Default for ProcHandle {
    fn default() -> Self {
        Self { 0: ProcessHandle::null_type() }
    }
}

unsafe impl Send for ProcHandle {}

unsafe impl Sync for ProcHandle {}
