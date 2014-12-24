#![feature(phase)]

extern crate docopt;
extern crate serialize;
extern crate "rustc-serialize" as rustc_serialize;
#[phase(plugin)] extern crate docopt_macros;

use std::io::fs;
use docopt::Docopt;

docopt!(Args deriving Show, "
Usage: ls [<path>...]
");

fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    println!("Just got {}", args.arg_path);
    let dir = Path::new(".");
    let contents = fs::readdir(&dir);
    match contents {
        Ok(results) => { process(results); }
        Err(e) => ()
    }
}

fn process(results: Vec<Path>) {
    for entry in results.iter() {
        println!("{}", entry.to_c_str());
    }
}
