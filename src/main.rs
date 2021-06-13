use std::{
    env::var,
    fs::{create_dir_all, File, OpenOptions},
    io::{BufRead, BufReader, Seek, SeekFrom, Write},
    path::Path,
    process::Command,
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
        Cmd::Add { programs } => add(programs),
    };
}

fn add(new_programs: Vec<String>) {
    let parent_path = Path::new(var("HOME").expect("HOME not defined").as_str())
        .join(".nixpkgs")
        .join("programs");

    create_dir_all(&parent_path).expect("Couldn't create directories");

    let file_path = parent_path.join("auto.txt");

    let mut file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(&file_path)
        .expect("Couldn't open config file");

    let mut programs = BufReader::new(&file)
        .lines()
        .map(|line| line.expect("Couldn't read line"))
        .collect::<Vec<_>>();

    let mut new_programs = new_programs.clone();

    programs.append(&mut new_programs);
    programs.sort();
    programs.dedup();

    let total_program_count = programs.len();

    clear_file(&mut file);

    for line in programs.into_iter() {
        println!("{}", format!("Saving program {}", line).green());

        file.write_all((line + "\n").as_bytes())
            .expect("can't write");
    }

    println!(
        "{}",
        format!(
            "Wrote {} lines to {}",
            total_program_count,
            file_path.to_str().unwrap()
        )
        .blue()
    );

    rebuild_system();
}

fn clear_file(file: &mut File) {
    file.set_len(0).expect("Couldn't truncate existing file");
    file.seek(SeekFrom::Start(0))
        .expect("Couldn't seek to beginning of file");
}

fn rebuild_system() {
    let mut cmd = Command::new("darwin-rebuild");
    cmd.arg("switch");

    let exit = cmd
        .spawn()
        .expect("Couldn't start darwin-rebuild")
        .wait()
        .expect("System rebuild failed")
        .code()
        .expect("Command didn't return an exit code");

    assert_eq!(exit, 0, "Command didn't complete with exit code 0");
}
