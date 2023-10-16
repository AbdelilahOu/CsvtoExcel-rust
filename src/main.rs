use std::env;

fn main() {
    // get and check for args
    let args: Vec<String> = env::args().collect();
    if &args.len() != 2 {
        panic!("Invalid number of arguments");
    }
    // get data folder path
    let data_folder = &args[1];
    // read folder
}
