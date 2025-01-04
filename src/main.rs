use clap::{Command, Arg};
use std::process::{Command as StdCommand, Stdio};
use colored::*;
use std::io::{BufReader, BufRead};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = Command::new("MoonCrate")
        .version(VERSION)
        .about("MoonCrate is open-source all-in-one unified package manager for everything")
        .subcommand(
            Command::new("install")
                .about("Install a package")
                .arg(Arg::new("package").required(true).help("Package to install")),
        )
        .subcommand(
            Command::new("remove")
                .about("Remove a package")
                .arg(Arg::new("package").required(true).help("Package to remove")),
        )
        .subcommand(
            Command::new("update-packages")
                .about("Update packages")
                .arg(Arg::new("package").required(true).help("Package to update")),
        )
        .subcommand(
            Command::new("update")
                .about("Full system update")
        )
        .subcommand(
            Command::new("search")
                .about("Search packages")
                .arg(Arg::new("package").required(true).help("Packages to search")),
        )
        .subcommand(
            Command::new("list")
                .about("Lists all packages"),
        )
        .subcommand(
            Command::new("about")
                .about("About MoonCrate"),
        )
        .get_matches();
    match_subcommands(&matches);
}

fn match_subcommands(matches: &clap::ArgMatches) {
    if let Some(matches) = matches.subcommand_matches("install") {
        let package = matches.get_one::<String>("package").unwrap();
        install_packages(package);
    } else if let Some(matches) = matches.subcommand_matches("remove") {
        let package = matches.get_one::<String>("package").unwrap();
        remove_packages(package);
    } else if let Some(matches) = matches.subcommand_matches("update-packages") {
        let package = matches.get_one::<String>("package").unwrap();
        update_packages(package);
    } else if let Some(matches) = matches.subcommand_matches("search") {
        let package = matches.get_one::<String>("package").unwrap();
        search_packages(package);
    } else if let Some(_matches) = matches.subcommand_matches("update") {
        update();
    } else if let Some(_matches) = matches.subcommand_matches("list") {
        list_packages();
    } else if let Some(_matches) = matches.subcommand_matches("about") {
        about_mooncrate();
    }
}

fn install_packages(package: &str) {
    println!(
        "{} {} {}",
        "::".green().bold(),
        format!("Installing {} and Synchronizing package databases", package.green().italic()).bold(),
        ""
    );
    let mut command = StdCommand::new("pkcon")
        .arg("install")
        .arg(package)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to install packages");
    if let Some(stdout) = command.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            match line {
                Ok(line_content) => println!("{}", line_content),
                Err(err) => eprintln!("Error reading line: {}", err),
            }
        }
    }
}

fn remove_packages(package: &str) {
    println!(
        "{} {} {}",
        "::".green().bold(),
        format!("Removing {} and Synchronizing package databases", package.green().italic()).bold(),
        ""
    );
    let mut command = StdCommand::new("pkcon")
        .arg("remove")
        .arg(package)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to remove packages");
    if let Some(stdout) = command.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            match line {
                Ok(line_content) => println!("{}", line_content),
                Err(err) => eprintln!("Error reading line: {}", err),
            }
        }
    }
}

fn update_packages(package: &str) {
    println!(
        "{} {} {}",
        "::".green().bold(),
        format!("Updating {} and Synchronizing package databases", package.green().italic()).bold(),
        ""
    );
    let mut command = StdCommand::new("pkcon")
        .arg("update")
        .arg(package)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to update packages");
    if let Some(stdout) = command.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            match line {
                Ok(line_content) => println!("{}", line_content),
                Err(err) => eprintln!("Error reading line: {}", err),
            }
        }
    }
}

fn update() {
    println!(
        "{} {}",
        "::".green().bold(),
        "Starting full system update".bold()
    );

    let mut command = StdCommand::new("pkcon")
        .arg("update")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to search packages");

    if let Some(stdout) = command.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            match line {
                Ok(line_content) => println!("{}", line_content),
                Err(err) => eprintln!("Error reading line: {}", err),
            }
        }
    } else {
        eprintln!("Failed to capture command output.");
    }
}

fn search_packages(package: &str) {
    println!(
        "{} {} {}",
        "::".green().bold(),
        format!("Searching {} package databases", package.green().italic()).bold(),
        ""
    );
    let mut command = StdCommand::new("pkcon")
        .arg("search")
        .arg(package)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to search packages");
    if let Some(stdout) = command.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            match line {
                Ok(line_content) => println!("{}", line_content),
                Err(err) => eprintln!("Error reading line: {}", err),
            }
        }
    }
}

fn list_packages() {
    let mut command = StdCommand::new("pkcon")
        .arg("list")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to list all packages");
    if let Some(stdout) = command.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            match line {
                Ok(line_content) => println!("{}", line_content),
                Err(err) => eprintln!("Error reading line: {}", err),
            }
        }
    }
}

fn about_mooncrate() {
    let description = "MoonCrate is an open-source, all-in-one unified package manager for everything.";
    let author = "Atthun Seeran R.A <idkatthun@gmail.com>";

    println!("{}", "MoonCrate - Package Manager for Lunaris".bold().green());
    println!("Version: {}", VERSION);
    println!("Description: {}", description);
    println!("Author: {}", author);
}
