use std::fs::File;

use colored::Colorize;

use crate::files::{get_programs, write_programs};

pub(crate) fn add(new_programs: Vec<String>, file: &mut File) {
    let mut programs = get_programs(file);
    combine_sorted(&mut programs, new_programs);

    let total_program_count = programs.len();

    write_programs(programs, file);
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
