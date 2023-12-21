use process_memory::ProcessHandle;

#[derive(Clone, Copy)]
pub struct ProcHandle(pub ProcessHandle);

unsafe impl Send for ProcHandle {}

unsafe impl Sync for ProcHandle {}
