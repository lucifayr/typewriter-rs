use std::{
    io::{self, Read, Write},
    thread,
    time::Duration,
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct TypeWriterArgs {
    #[arg(short, long)]
    time: Option<u64>,
}

fn main() {
    let args = TypeWriterArgs::parse();

    let mut text = String::new();
    io::stdin()
        .read_to_string(&mut text)
        .expect("Failed to read from stdin");

    let time = Duration::from_millis(args.time.unwrap_or(50));
    for char in text.chars() {
        print!("{char}");
        io::stdout().flush().expect("Failed to flush stdout");
        thread::sleep(time)
    }
}
