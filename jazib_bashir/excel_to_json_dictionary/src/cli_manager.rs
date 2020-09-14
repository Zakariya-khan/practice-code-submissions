use clap::{App, Arg};
use std::{path::Path, path::PathBuf};

fn check_if_excel(file: &str) -> bool {
  let sce = PathBuf::from(file);
  match sce.extension().and_then(|s| s.to_str()) {
    Some("xlsx") | Some("xlsm") | Some("xlsb") | Some("xls") => true,
    _ => false,
  }
}

fn path_exists(file: &str) -> bool {
  Path::new(file).exists()
}

pub struct Args {
  pub file: String,
}

impl Args {
  pub fn parse() -> Self {
    // New instance of application
    let matches = App::new("XLSX to JSON")
      .version("0.1.0")
      .about("Transofrm data in Excel files into the JSON format")
      .arg(
        Arg::with_name("PATH")
          .long("path")
          .short("p")
          .required(true)
          .takes_value(true)
          .help("Sets the XLSX file to use"),
      )
      .get_matches();

    // Calling .unwrap() is safe here because "FIELD" is required (if "FIELD" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    let file = matches.value_of("PATH").expect("Please provide a xlsx file");

    if !path_exists(file) {
      panic!("File doesn't exist");
    }

    if !check_if_excel(file) {
      panic!("Expecting an XLSX file");
    }

    // returning Args struct
    Self { file: file.into() }
  }
}
