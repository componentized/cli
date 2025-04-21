#![no_main]

use crate::exports::wasi::cli::stdout::Guest;
use crate::wasi::cli::stderr::get_stderr;
use crate::wasi::io::streams::OutputStream;

pub(crate) struct StdoutToStderr;

impl Guest for StdoutToStderr {
    fn get_stdout() -> OutputStream {
        get_stderr()
    }
}

wit_bindgen::generate!({
    path: "../../wit",
    world: "stdout-to-stderr",
    generate_all
});

export!(StdoutToStderr);
