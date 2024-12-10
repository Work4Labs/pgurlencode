use pgrx::prelude::*;
use urlencoding::{encode, encode_binary};

::pgrx::pg_module_magic!();

#[pg_extern]
fn urlencode(input: &str) -> String {
    encode(input).to_string()
}

#[pg_extern]
fn urlencode_bin(input: &[u8]) -> String {
    encode_binary(input).to_string()
}
