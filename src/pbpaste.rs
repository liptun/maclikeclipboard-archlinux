use clipboard_ext::prelude::*;
use clipboard_ext::x11_fork::ClipboardContext;

fn main() {
    let mut ctx = ClipboardContext::new().unwrap();
    print!("{}", ctx.get_contents().unwrap());
}
