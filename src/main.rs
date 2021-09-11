mod frontend;

use std::{env, fs, path::Path};

fn main() {
    let file_name = env::args().nth(1).expect("Provide a file name");
    let path = Path::new(&file_name);
    let mut i = fs::read_to_string(path).expect("Invalid file");
    i.insert(0, '\n');
    let (rest, program) = frontend::instruction::program(&i).expect("Could not parse the program");
    dbg!(program);
    assert_eq!(rest, "");
}
