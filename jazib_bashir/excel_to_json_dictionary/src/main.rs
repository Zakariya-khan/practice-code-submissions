pub mod cli_manager;
pub mod excel_parser;

use cli_manager::Args;

fn main() {
	// get file argument from cli manager
	let args = Args::parse();
	let Args { file } = args;
	// pass file to excel parser for json dictionary
	excel_parser::parse_into_json(&file);
}
