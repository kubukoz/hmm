use std::{
    env::var,
    fs::{create_dir_all, File, OpenOptions},
    io::{BufRead, BufReader, Read, Seek, SeekFrom, Write},
    path::Path,
};

use colored::Colorize;
use rnix::{
    parse as parse_nix,
    types::{List, TypedNode},
};

pub(crate) fn open_rw_or_create(file_path: &std::path::PathBuf) -> File {
    OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(file_path)
        .expect("Couldn't open config file")
}

fn clear_file(file: &mut File) {
    file.set_len(0).expect("Couldn't truncate existing file");
    file.seek(SeekFrom::Start(0))
        .expect("Couldn't seek to beginning of file");
}

pub(crate) fn ensure_config_file() -> std::path::PathBuf {
    let parent_path = Path::new(var("HOME").expect("HOME not defined").as_str())
        .join(".nixpkgs")
        .join("programs");

    create_dir_all(&parent_path).expect("Couldn't create directories");
    parent_path.join("auto.txt")
}

pub(crate) fn get_programs(file: &mut File) -> Vec<String> {
    // let mut str = String::new();
    // file.read_to_string(&mut str);

    // let parsed = parse_nix(str.as_str()).root();
    // List::cast(parsed.node().to_owned());

    BufReader::new(file)
        .lines()
        .map(|line| line.expect("Couldn't read line"))
        .collect::<Vec<_>>()
}
pub(crate) fn write_programs(programs: Vec<String>, file: &mut File) {
    clear_file(file);

    for line in programs.into_iter() {
        println!("{}", format!("Saving program {}", line).green());

        file.write_all((line + "\n").as_bytes())
            .expect("can't write to file");
    }
}
