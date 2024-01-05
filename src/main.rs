

// The user has lost their Rust source file. 
// Let's reconstruct the file from the previous discussions and code provided.

use clap::{App, Arg, SubCommand};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    let matches = App::new("Table Manager")
        .version("1.0")
        .author("Your Name")
        .about("Manages simple tables")
        .arg(Arg::with_name("table_name")
             .help("The name of the table file to operate on")
             .takes_value(true)
             .required(true))
        .subcommand(SubCommand::with_name("add")
            .about("Adds data to a specific cell")
            .arg(Arg::with_name("CELL")
                 .help("The cell to add data to (e.g., A1)")
                 .required(true)
                 .index(1))
            .arg(Arg::with_name("DATA")
                 .help("The data to add to the cell")
                 .required(true)
                 .index(2)))
        // ... (rest of the code)
        .subcommand(SubCommand::with_name("show")
            .about("Displays the current state of the table"))
        .subcommand(SubCommand::with_name("new")
            .about("Creates a new table"))
        .get_matches();

    let table_name = matches.value_of("table_name").expect("Table name is required");

    match matches.subcommand() {
        ("new", Some(_)) => {
            create_new_table(table_name)?;
        },
        ("add", Some(add_matches)) => {
            let cell = add_matches.value_of("CELL").unwrap();
            let data = add_matches.value_of("DATA").unwrap();
            add_data_to_table(table_name, cell, data)?;
        },
        ("show", Some(_)) => {
            display_table(table_name)?;
        },
        _ => unreachable!(),
    }

    Ok(())
}

fn create_new_table(file_name: &str) -> io::Result<()> {
    let path = Path::new(file_name);
    File::create(path)?;
    Ok(())
}

fn add_data_to_table(file_name: &str, cell: &str, data: &str) -> io::Result<()> {
    let mut table = load_table(file_name)?;
    table.insert(cell.to_string(), data.to_string());
    save_table(file_name, &table)
}

fn load_table(file_name: &str) -> io::Result<HashMap<String, String>> {
    let mut table = HashMap::new();
    let path = Path::new(file_name);
    if path.exists() {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        for (i, line) in reader.lines().enumerate() {
            let line = line?;
            for (j, data) in line.split('\t').enumerate() {
                if !data.is_empty() {
                    let cell_name = format!("{}{}", (b'A' + j as u8) as char, i + 1);
                    table.insert(cell_name, data.to_string());
                }
            }
        }
    }
    Ok(table)
}

fn save_table(file_name: &str, table: &HashMap<String, String>) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_name)?;
    // Assuming a maximum table size (e.g., 10x10)
    for i in 0..10 {
        for j in 0..10 {
            let cell_name = format!("{}{}", (b'A' + j as u8) as char, i + 1);
            if let Some(data) = table.get(&cell_name) {
                write!(file, "{}\t", data)?;
            } else {
                write!(file, "\t")?;
            }
        }
        writeln!(file)?;
    }
    Ok(())
}
fn display_table(file_name: &str) -> io::Result<()> {
    let table = load_table(file_name)?;

    // Determine the size of the table
    let max_row = table.keys().filter_map(|k| k.chars().last()?.to_digit(10)).max().unwrap_or(0) as usize;
    let max_column = table.keys().filter_map(|k| k.chars().next()).max().unwrap_or('A') as u8 - b'A' + 1;

    // Print column headers without borders
    print!("     "); // Top left corner space for alignment
    for c in 0..max_column {
        print!("    {:^4}   ", (b'A' + c) as char);
    }
    println!();

    // Print top border of the table
    print!("     +"); // Left corner of the table
    for _ in 0..max_column {
        print!("----------+");
    }
    println!();

    // Print rows with borders
    for r in 1..=max_row {
        // Print row number and left border
        print!("{:<5}|", r);

        for c in 0..max_column {
            let cell_name = format!("{}{}", (b'A' + c) as char, r);
            let data = table.get(&cell_name).map_or("", String::as_str);
            print!("{:^10}|", data); // Center-align the data in the cell
        }
        println!();

        // Print row border
        print!("     +");
        for _ in 0..max_column {
            print!("----------+");
        }
        println!();
    }
    Ok(())
}

