use clipboard_ext::prelude::*;
use clipboard_ext::x11_fork::ClipboardContext;
use std::io::{self, Read};

fn main() {
    let mut ctx = ClipboardContext::new().unwrap();
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    ctx.set_contents(input).unwrap();
}
