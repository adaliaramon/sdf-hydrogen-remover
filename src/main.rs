extern crate clap;

use std::fs;

use clap::{App, Arg};

fn main() {
    let app = App::new("SDF Hydrogen remover")
        .author("Ramon Adàlia")
        .version("0.1.0")
        .arg(
            Arg::with_name("sdf")
                .required(true)
                .takes_value(true)
                .value_name("SDF"),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .required(false)
                .takes_value(true)
                .value_name("output"),
        );

    let matcher = app.get_matches();
    let sdf = matcher.value_of("sdf").unwrap();
    let content = fs::read_to_string(sdf).unwrap();
    let split: Vec<&str> = content.split("\n").collect();

    let information_row = split.get(3).unwrap();
    let n_atoms: usize = information_row[0..3].trim().parse().unwrap();
    let n_bonds: usize = information_row[3..6].trim().parse().unwrap();

    let mut atom_block = String::new();
    let mut cum_h_sums = Vec::with_capacity(n_atoms);
    let mut is_hydrogen = Vec::with_capacity(n_atoms);
    let mut sum = 0;
    for i in 4..4 + n_atoms {
        cum_h_sums.push(sum);
        let line = split.get(i).unwrap();
        let atom = line[30..32].trim();
        if atom == "H" {
            sum += 1;
            is_hydrogen.push(true);
        } else {
            atom_block += &format!("{}\n", line);
            is_hydrogen.push(false);
        }
    }

    let mut bond_block = String::new();
    let mut new_n_bonds = 0;
    for i in 4 + n_atoms..4 + n_atoms + n_bonds {
        let line = split.get(i).unwrap();
        let root: usize = line[0..3].trim().parse().unwrap();
        if *is_hydrogen.get(root - 1).unwrap() {
            continue;
        }
        let target: usize = line[3..6].trim().parse().unwrap();
        if *is_hydrogen.get(target - 1).unwrap() {
            continue;
        }
        new_n_bonds += 1;
        let new_root = root - cum_h_sums.get(root - 1).unwrap();
        let new_target = target - cum_h_sums.get(target - 1).unwrap();
        let rest = &line[6..];
        bond_block += &format!("{: >3}{: >3}{}\n", new_root, new_target, rest);
    }

    let mut string = String::new();
    string += &format!(
        "{}\n{}\n{}\n",
        split.get(0).unwrap(),
        split.get(1).unwrap(),
        split.get(2).unwrap()
    );
    let rest = &information_row[6..];
    string += &format!("{: >3}{: >3}{}\n", n_atoms - sum, new_n_bonds, rest);
    string += &atom_block;
    string += &bond_block;
    for line in split[4 + n_atoms + n_bonds..].iter() {
        string += &format!("{}\n", line);
    }
    string.pop();

    if matcher.is_present("output") {
        let output = matcher.value_of("output").unwrap();
        fs::write(output, string).unwrap();
    } else {
        print!("{}", string);
    }
}
