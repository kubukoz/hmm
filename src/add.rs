use std::fs::File;

use colored::Colorize;

use crate::{
    files::{read_file, write_file},
    nix::{nixfmt_run, parse_nix_string_list, render_nix_string_list},
};

pub(crate) fn add(new_programs: Vec<String>, file: &mut File) {
    let mut programs = parse_nix_string_list(read_file(file));
    combine_sorted(&mut programs, new_programs);

    let total_program_count = programs.len();

    for line in programs.iter() {
        println!("{}", format!("Saving program {}", line).green());
    }

    write_file(nixfmt_run(render_nix_string_list(&programs)), file);
    print_summary(total_program_count);
}

fn print_summary(total_program_count: usize) {
    println!("{}", format!("Wrote {} lines", total_program_count,).blue());
}

fn combine_sorted<T: Ord + Clone>(old: &mut Vec<T>, new: Vec<T>) {
    old.append(&mut new.clone());
    old.sort();
    old.dedup();
}
