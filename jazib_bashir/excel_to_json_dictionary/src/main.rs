pub mod cli_manager;

use cli_manager::Args;

fn main() {
	let args = Args::parse();
	let Args { file } = args;
	println!("File Path! {:?}", file);
}
