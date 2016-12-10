use std::thread;
use std::time::Duration;
use std::io::{self, Write};

fn main() {
    for x in 0..10 {
        thread::sleep(Duration::from_millis(20));
        io::stdout().write_fmt(format_args!("Chunk from pseudo_mondpaint: {}\n", x)).unwrap();
        io::stdout().flush().unwrap();
    }
}
