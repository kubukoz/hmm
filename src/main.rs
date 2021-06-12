use std::{
    env::args,
    fs::File,
    io::{BufRead, BufReader, Write},
};

fn main() {
    let new_program = args().nth(1).expect("Missing program argument");

    let file = File::open("./programs.txt").expect("Couldn't open config file");

    let mut lines = BufReader::new(&file)
        .lines()
        .map(|line| line.expect("Couldn't read line"))
        .collect::<Vec<_>>();

    lines.insert(lines.len(), new_program);
    lines.sort();
    lines.dedup();

    let mut file = File::create("./programs.txt").expect("Couldn't open config file for writing");

    let len = lines.len();

    for line in lines.into_iter() {
        file.write_all((line + "\n").as_bytes())
            .expect("can't write");
    }

    println!("Wrote {} lines to file", len);
}
