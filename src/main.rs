use std::{env::args, fs::read_dir};

fn main() {
    // get and check for args
    let args: Vec<String> = args().collect();
    // check for args
    match &args.len() {
        2 => {
            // get data folder path
            let data_folder = &args[1];
            // read folder
            let files = read_dir(data_folder).unwrap();
            // iterate through files
            for file in files {
                println!("Name: {}", file.unwrap().path().display())
            }
        }
        _ => panic!("Invalid number of arguments"),
    }
}
