use std::io::fs;

fn main() {
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
