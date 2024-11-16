use std::path::PathBuf;

const GO_PROGRAM_PATH: &str = env!("GO_PROGRAM");

/// Runs the go program and returns the output.
pub fn get_go_program() -> PathBuf {
    let runfiles = runfiles::Runfiles::create().expect("failed to create runfiles");
    let go_program =
        runfiles::rlocation!(runfiles, GO_PROGRAM_PATH).expect("failed to locate go program");
    go_program
}
