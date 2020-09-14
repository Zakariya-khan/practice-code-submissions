# EXCEL TO JSON DICTIONARY

### To run the local development build:
- Run the current package

        cargo run -- -p <XLSX file path>

  Sample XLSX File is available in assets folder. This utility will create json file in `output` folder.

### Build:
- Compile the current package. Cargo has two main profiles:

    the dev profile Cargo uses when you run 
    
        cargo build 

    and the release profile Cargo uses when you run

        cargo build --release

    This command creates an executable file in `target/debug/<project_name>` rather than in your current directory. You can run the executable with this command:
    
        ./target/debug/<project_name> -p <XLSX file path>