use std::{env::args, fs::read_dir, path::Path};

#[derive(Debug)]
struct CsvFilesData {
    path: Box<Path>,
    name: String,
    sheet_name: String,
}

fn to_pascale_case(name: String) -> String {
    if name.is_empty() {
        return name;
    }

    let mut result = String::from("");

    if name.contains("_") {
        for subname in name.split("_") {
            result = result + &to_pascale_case(subname.to_string());
        }
    } else {
        let first_chart = name.chars().nth(0).unwrap().to_uppercase().to_string();
        result = first_chart + &name[1..];
    }

    return result;
}

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
            // data
            let mut files_data: Vec<CsvFilesData> = Vec::new();
            // iterate through files
            for file in files {
                match file {
                    Ok(file) => {
                        let file_path = Path::new(data_folder).join(file.path());
                        let file_name_with_ext = file.file_name().into_string().ok();
                        match file_name_with_ext {
                            Some(file_name_with_ext) => {
                                let file_name =
                                    file_name_with_ext.split(".").next().unwrap().to_string();
                                files_data.push(CsvFilesData {
                                    path: file_path.as_path().into(),
                                    name: file_name.clone(),
                                    sheet_name: to_pascale_case(file_name),
                                })
                            }
                            None => {
                                println!("Invalid file name");
                                continue;
                            }
                        }
                    }
                    Err(e) => panic!("{}", e),
                }
            }
            println!("{:?}", files_data)
        }
        _ => panic!("Invalid number of arguments"),
    }
}
