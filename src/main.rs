use std::{
    env::var,
    fs::{create_dir_all, File, OpenOptions},
    io::{BufRead, BufReader, Seek, SeekFrom, Write},
    path::Path,
    process::Command,
    usize,
};

use colored::*;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "Home Manager Manager",
    about = "Manages your Home Manager config",
    version = env!("CARGO_PKG_VERSION"),

)]
enum Cmd {
    #[structopt(about = "Installs a program")]
    Add {
        #[structopt(required = true)]
        programs: Vec<String>,
    },
}

fn main() {
    let command = Cmd::from_args();

    match command {
        Cmd::Add { programs } => {
            let file_path = find_config_file();

            let mut file = open_rw_or_create(&file_path);

            add(programs, &mut file);

            rebuild_system();
        }
    };
}

fn add(new_programs: Vec<String>, file: &mut File) {
    let mut programs = get_programs(&file);
    combine_sorted(&mut programs, new_programs);

    let total_program_count = programs.len();

    clear_file(file);

    write_programs(programs, file);
    print_summary(total_program_count);
}

fn print_summary(total_program_count: usize) {
    println!("{}", format!("Wrote {} lines", total_program_count,).blue());
}

fn write_programs(programs: Vec<String>, file: &mut File) {
    for line in programs.into_iter() {
        println!("{}", format!("Saving program {}", line).green());

        file.write_all((line + "\n").as_bytes())
            .expect("can't write to file");
    }
}

fn find_config_file() -> std::path::PathBuf {
    let parent_path = Path::new(var("HOME").expect("HOME not defined").as_str())
        .join(".nixpkgs")
        .join("programs");

    create_dir_all(&parent_path).expect("Couldn't create directories");
    parent_path.join("auto.txt")
}

fn open_rw_or_create(file_path: &std::path::PathBuf) -> File {
    OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(file_path)
        .expect("Couldn't open config file")
}

fn get_programs(file: &File) -> Vec<String> {
    BufReader::new(file)
        .lines()
        .map(|line| line.expect("Couldn't read line"))
        .collect::<Vec<_>>()
}

fn combine_sorted<T: Ord + Clone>(old: &mut Vec<T>, new: Vec<T>) {
    old.append(&mut new.clone());
    old.sort();
    old.dedup();
}

fn clear_file(file: &mut File) {
    file.set_len(0).expect("Couldn't truncate existing file");
    file.seek(SeekFrom::Start(0))
        .expect("Couldn't seek to beginning of file");
}

fn rebuild_system() {
    let exit = Command::new("darwin-rebuild")
        .arg("switch")
        .spawn()
        .expect("Couldn't start darwin-rebuild")
        .wait()
        .expect("System rebuild failed")
        .code()
        .expect("Command didn't return an exit code");

    assert_eq!(exit, 0, "System rebuild failed");
}
