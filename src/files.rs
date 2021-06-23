use std::{
    env::var,
    fs::{create_dir_all, File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    path::Path,
};

use colored::Colorize;

use crate::nix::{nixfmt_run, parse_nix_string_list, render_string_list};

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
    parent_path.join("auto.nix")
}

pub(crate) fn get_programs(file: &mut File) -> Vec<String> {
    let mut str = String::new();
    file.read_to_string(&mut str).expect("Couldn't read file");

    parse_nix_string_list(str)
}

pub(crate) fn write_programs(programs: Vec<String>, file: &mut File) {
    let formatted = nixfmt_run(render_string_list(&programs));

    for line in programs.iter() {
        println!("{}", format!("Saving program {}", line).green());
    }

    clear_file(file);
    file.write_all(formatted.as_bytes())
        .expect("Couldn't write to file");
}
