use std::{fs::File, io::{Read, Error}, path::Path};

fn main() -> std::io::Result<()> {
    let mut data = String::new();
    let mut file = File::open("example.mbox")?;
    let _ret = file.read_to_string(&mut data);
    println!("{}", data);
    Ok(())
}
