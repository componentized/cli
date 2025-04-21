#![no_main]

use crate::exports::wasi::cli::stderr::Guest;
use crate::wasi::cli::stdout::get_stdout;
use crate::wasi::io::streams::OutputStream;

pub(crate) struct StderrToStdout;

impl Guest for StderrToStdout {
    fn get_stderr() -> OutputStream {
        get_stdout()
    }
}

wit_bindgen::generate!({
    path: "../../wit",
    world: "stderr-to-stdout",
    generate_all
});

export!(StderrToStdout);
