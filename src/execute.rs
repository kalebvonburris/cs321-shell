use std::ffi::{CString, c_char};

use super::Command;

/// Executes a command in a child process using `fork` and `execvp`.
///
/// # Safety
///
/// This function is unsafe because it uses `libc::fork` and `libc::execvp`.
/// Special care must be taken to ensure given commands and arguments are valid and properly formatted.
///
/// ```
/// use cs321_shell::execute;
/// use cs321_shell::Command;
///
/// let command = Command {
///     command: "ls".to_string(),
///     args: vec!["-l".to_string(), "-a".to_string()],
///     delimiter: Default::default(),
/// };
/// let result = unsafe { execute(command) };
/// if result != 0 {
///    return;
/// }
/// ```
pub unsafe fn execute(command: Command) -> i32 {
    let id = unsafe { libc::fork() };

    // Parent case
    if id != 0 {
        return id;
    }

    let program = CString::new(command.command.as_bytes()).unwrap();
    let program_ptr = program.as_ptr();

    let arg_cstrings: Vec<CString> = command
        .args
        .iter()
        .map(|arg| CString::new(arg.as_bytes()).unwrap())
        .collect();

    let mut arg_ptrs: Vec<*const c_char> = Vec::with_capacity(command.args.len() + 2);
    arg_ptrs.push(program_ptr);

    for arg in &arg_cstrings {
        arg_ptrs.push(arg.as_ptr());
    }

    arg_ptrs.push(std::ptr::null()); // Null-terminate the array

    let argv_ptr = arg_ptrs.as_ptr();

    // Child case
    let result = unsafe { libc::execvp(program_ptr, argv_ptr) };

    // If execvp fails, print the error and return 0
    eprintln!("Error: execvp failed with error code: {}", result);
    unsafe {
        libc::exit(-1);
    } // Exit the child process with an error code
}
