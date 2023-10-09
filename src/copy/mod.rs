use std::fs;
use std::env; 

struct Argument {
    input_file: String,
    output_file: String,
}

fn get_file_content(file: String) -> String {
    match fs::read_to_string(file) {
        Ok(v) => v,
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

pub fn copy_file(input_file: &str, output_file: &str) {
    let data = get_file_content(String::from(input_file));

    match fs::write(output_file, data) {
        Ok(_) => (),
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

pub fn get_args() {
    let args: Argument = {
        let args: Vec<String> = env::args().skip(1).collect();
        
        Argument { input_file: args[0].clone(), output_file: args[1].clone() }
    };

    copy_file(args.input_file.as_str(), args.output_file.as_str());
}