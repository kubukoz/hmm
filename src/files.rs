use std::{
    env::var,
    fs::{File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    path::Path,
};

use crate::nix::nixfmt_run;

pub(crate) fn open_rw_or_create(file_path: &std::path::PathBuf) -> File {
    println!("Reading {}", file_path.display());
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

pub(crate) fn root_path() -> std::path::PathBuf {
    Path::new(var("HOME").expect("HOME not defined").as_str()).join(".nixpkgs")
}

pub(crate) fn read_file(file: &mut File) -> String {
    let mut str = String::new();
    file.read_to_string(&mut str).expect("Couldn't read file");
    str
}

pub(crate) fn write_file(text: String, file: &mut File) {
    let formatted = nixfmt_run(text);

    clear_file(file);
    file.write_all(formatted.as_bytes())
        .expect("Couldn't write to file");
}
