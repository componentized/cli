#![no_main]

use crate::exports::wasi::cli::stderr::{ErrorCode, Guest};
use crate::wasi::cli::stdout;

pub(crate) struct StderrToStdout;

impl Guest for StderrToStdout {
    #[doc = "/ Write the given stream to stderr."]
    #[doc = "/"]
    #[doc = "/ If the stream\'s writable end is dropped this function will either return"]
    #[doc = "/ success once the entire contents of the stream have been written or an"]
    #[doc = "/ error-code representing a failure."]
    #[doc = "/"]
    #[doc = "/ Otherwise if there is an error the readable end of the stream will be"]
    #[doc = "/ dropped and this function will return an error-code."]
    #[allow(async_fn_in_trait)]
    fn write_via_stream(
        data: wit_bindgen::rt::async_support::StreamReader<u8>,
    ) -> wit_bindgen::rt::async_support::FutureReader<Result<(), ErrorCode>> {
        stdout::write_via_stream(data)
    }
}

wit_bindgen::generate!({
    path: "../../wit",
    world: "stderr-to-stdout",
    merge_structurally_equal_types: true,
    generate_all
});

export!(StderrToStdout);
