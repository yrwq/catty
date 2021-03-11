use std::env::args;
use std::fs::File;
use std::io::Read;
use std::process;

// Languages
mod c;
mod rs;

fn main() {

    // Collect command line arguments
    let args: Vec<String> = args().collect();

    if args.len() > 1 {
    } else {
        print!("\nUsage: catty <file>\n\n");
        process::exit(0x0100);
    }

    let filename = &args[1];
    match File::open(filename) {

        // The file is open
        Ok(mut file) => {
            let mut content = String::new();

            // Read file content to a string
            file.read_to_string(&mut content).unwrap();

            // Detect file type by looking for the suffix
            if filename.ends_with(".rs") {
                rs::highlight(&content);
            } else if filename.ends_with(".c") {
                c::highlight(&content);
            }

            // The file is automatically closed when is goes out of scope.
        },

        // Error handling.
        Err(error) => {
            println!("Error opening file {}", error);
        },
    }
}
