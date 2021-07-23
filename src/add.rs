use std::fs::File;

use colored::Colorize;

use crate::{
    files::{read_file, write_file},
    nix::{nixfmt_run, parse_nix_string_list, render_nix_string_list},
    types::{Add, UpdateKind, UpdateResult},
};

pub(crate) fn add(new_programs: &Vec<String>, file: &mut File) -> UpdateResult<Add> {
    let old_programs = parse_nix_string_list(read_file(file));
    let mut programs = old_programs.clone();

    combine_sorted(&mut programs, &new_programs);

    let total_program_count = programs.len();

    for line in programs.iter() {
        println!("{}", format!("Saving program {}", line).green());
    }

    write_file(nixfmt_run(render_nix_string_list(&programs)), file);
    print_summary(total_program_count);

    let updates = new_programs
        .to_owned()
        .into_iter()
        .filter(|p| !old_programs.contains(p))
        .map(|p| Add { program: p })
        .collect();

    UpdateResult {
        updates,
        kind: UpdateKind::Add,
    }
}

fn print_summary(total_program_count: usize) {
    println!("{}", format!("Wrote {} lines", total_program_count,).blue());
}

fn combine_sorted<T: Ord + Clone>(old: &mut Vec<T>, new: &Vec<T>) {
    old.append(&mut new.clone());
    old.sort();
    old.dedup();
}
