use chrono::{Datelike, Timelike};
use std::fs::File;
use std::path::PathBuf;
use std::mem;

#[cfg(target_os = "windows")]
use std::os::windows::io::AsRawHandle;
#[cfg(target_os = "windows")]
use windows::Win32::Foundation::HANDLE;
#[cfg(target_os = "windows")]
use windows::Win32::System::Console::{SetStdHandle, STD_ERROR_HANDLE, STD_OUTPUT_HANDLE};

#[cfg(not(target_os = "windows"))]
use std::os::fd::AsRawFd;

#[cfg_attr(debug_assertions, allow(dead_code))]
pub fn activate_logging<P>(with_dir: P) -> anyhow::Result<()>
where 
    P: Into<PathBuf> + Clone
{
    let time_and_date = chrono::Local::now();
    let time_and_date_string = format![
        "{}-{}-{}_{}-{}-{}",
        time_and_date.year(),
        time_and_date.month(),
        time_and_date.day(),
        time_and_date.time().hour(),
        time_and_date.time().minute(),
        time_and_date.time().second()
    ];

    let stdout_file;
    let stderr_file;

    let stdout_file_name = format!["LOG__{}__STDOUT.txt", time_and_date_string.clone()];
    let stderr_file_name = format!["LOG__{}__STDERR.txt", time_and_date_string.clone()];

    // Create file for stdout
    stdout_file = match File::create(with_dir.clone().into().join(stdout_file_name.clone())) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Could not create the file for stdout: {:?}", err);
            return Err(anyhow::Error::msg(format!("Could not create the file for stdout: {:?}", err)));
        }
    };
    // Create file for stdout
    stderr_file = match File::create(with_dir.clone().into().join(stderr_file_name.clone())) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Could not create the file for stderr: {:?}", err);
            return Err(anyhow::Error::msg(format!("Could not create the file for stderr: {:?}", err)));
        }
    };

    // Use file descriptors on Unix
    #[cfg(not(target_os = "windows"))]
    {
        // First, get the file descriptors
        let stdout_file_fd = stdout_file.as_raw_fd();
        let stderr_file_fd = stderr_file.as_raw_fd();
        let stdout_fd = std::io::stdout().as_raw_fd();
        let stderr_fd = std::io::stderr().as_raw_fd();

        // Forget about the files, so they don't get deallocated!
        // They have to be available until the end of the program.
        mem::forget(stdout_file);
        mem::forget(stderr_file);

        // Now change the file handles and call it day.
        unsafe {
            let result_stdout = libc::dup2(stdout_file_fd, stdout_fd);
            if result_stdout == -1 {
                eprintln!("errno: {:?}", std::io::Error::last_os_error());
                return Err(std::io::Error::last_os_error().into());
            }
            let result_stderr = libc::dup2(stderr_file_fd, stderr_fd);
            if result_stderr == -1 {
                eprintln!("errno: {:?}", std::io::Error::last_os_error());
                return Err(std::io::Error::last_os_error().into());
            }
        }
    }

    // Use file handles on Windows
    #[cfg(target_os = "windows")]
    unsafe {
        // First, get the file handles
        let stdout_file_fh = stdout_file.as_raw_handle();
        let stderr_file_fh = stderr_file.as_raw_handle();

        // Forget about the files, so they don't get deallocated!
        // They have to be available until the end of the program.
        mem::forget(stdout_file);
        mem::forget(stderr_file);

        // Now change the file handles and call it day.
        match SetStdHandle(STD_OUTPUT_HANDLE, HANDLE(stdout_file_fh as isize)) {
            Ok(()) => {}
            Err(err) => {
                eprintln!("errno: {:?}", err);
                return Err(err.into());
            }
        }
        match SetStdHandle(STD_ERROR_HANDLE, HANDLE(stderr_file_fh as isize)) {
            Ok(()) => {}
            Err(err) => {
                eprintln!("errno: {:?}", err);
                return Err(err.into());
            }
        }
    }

    return Ok(());
}