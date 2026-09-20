//! Owned Windows Job Object creation and assignment.
use std::io;
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::System::JobObjects::{AssignProcessToJobObject, CreateJobObjectW};
use windows::Win32::System::Threading::{OpenProcess, PROCESS_SET_QUOTA, PROCESS_TERMINATE};
use windows::core::PCWSTR;
/// Job handle whose explicit lifetime defines descendant ownership.
pub(super) struct JobHandle(pub(super) HANDLE);
// SAFETY: Windows kernel handles may be used and closed from any thread.
unsafe impl Send for JobHandle {}
impl Drop for JobHandle {
    fn drop(&mut self) {
        let _ = unsafe { CloseHandle(self.0) };
    }
}
/// Creates a job and assigns its root process, closing handles on error.
pub(super) fn create_job(pid: u32) -> io::Result<JobHandle> {
    let job = unsafe { CreateJobObjectW(None, PCWSTR::null()) }.map_err(windows_error)?;
    let process = match unsafe { OpenProcess(PROCESS_SET_QUOTA | PROCESS_TERMINATE, false, pid) } {
        Ok(process) => process,
        Err(error) => {
            let _ = unsafe { CloseHandle(job) };
            return Err(windows_error(error));
        }
    };
    let assigned = unsafe { AssignProcessToJobObject(job, process) };
    let _ = unsafe { CloseHandle(process) };
    match assigned {
        Ok(()) => Ok(JobHandle(job)),
        Err(error) => {
            let _ = unsafe { CloseHandle(job) };
            Err(windows_error(error))
        }
    }
}
fn windows_error(error: windows::core::Error) -> io::Error {
    io::Error::other(error.to_string())
}
