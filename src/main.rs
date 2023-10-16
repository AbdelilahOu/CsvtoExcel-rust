use rust_xlsxwriter::{self, Workbook};
use std::{env::args, fs::read_dir, path::Path};

#[derive(Debug)]
struct CsvFilesData {
    path: Box<Path>,
    name: String,
    sheet_name: String,
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
                        let file_name_with_ext = file.file_name().into_string().ok();
                        match file_name_with_ext {
                            Some(file_name_with_ext) => {
                                let file_name =
                                    file_name_with_ext.split(".").next().unwrap().to_string();
                                files_data.push(CsvFilesData {
                                    path: file.path().as_path().into(),
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
            // create new workbook
            let mut workbook = Workbook::new();
            // start printing
            for file_data in files_data {
                print_table_to_excel(&mut workbook, file_data)
            }

            workbook.save("output.xlsx").unwrap();
        }
        _ => panic!("Invalid number of arguments"),
    }
}

fn to_pascale_case(name: String) -> String {
    // retrn if name is empty
    if name.is_empty() {
        return name;
    }
    // create result
    let mut result = String::from("");
    // check if it has _ and uppercase first chart
    if name.contains("_") {
        for subname in name.split("_") {
            result = result + &to_pascale_case(subname.to_string());
        }
    } else {
        let first_chart = name.chars().nth(0).unwrap().to_uppercase().to_string();
        result = first_chart + &name[1..];
    }
    //
    return result;
}

fn print_table_to_excel(workbook: &mut Workbook, csc_file: CsvFilesData) {
    // create sheet
    let worksheet_result = workbook.add_worksheet().set_name(&csc_file.sheet_name);
    match worksheet_result {
        Ok(worksheet) => {}
        Err(err) => {
            panic!("coudnt create sheet from name: {:?}", err)
        }
    }
}
