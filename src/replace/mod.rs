use std::fs;
use std::env;

struct Argument {
    filename: String,
    target_word: String,
    new_word: String
}

pub fn replace_file(filename: &str, target_word: &str, new_word: &str) {
    let mut data = match fs::read_to_string(filename) {
        Ok(v) => v,
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    };

    data = data.replace(target_word, new_word);

    match fs::write(filename, data) {
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

        Argument { filename: args[0].clone(), target_word: args[1].clone(), new_word: args[2].clone() }
    };

    replace_file(args.filename.as_str(), args.target_word.as_str(), args.new_word.as_str());
}