use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
    process::exit,
};

pub fn create_dir_files(source_dir: PathBuf, destination: PathBuf) {
    match fs::read_dir(source_dir.clone()) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(file) => {
                        if file.file_type().unwrap().is_file() {
                            let mut source_file_path = source_dir.clone();
                            source_file_path.push(file.file_name());
                            let contents = match fs::read_to_string(&source_file_path) {
                                Ok(contents) => contents,
                                Err(_err) => {
                                    println!("Error while reading file at {:?}", &source_file_path);
                                    exit(1);
                                }
                            };

                            let mut destination_file_path = destination.clone();
                            destination_file_path.push(file.file_name());

                            let mut new_file = match OpenOptions::new()
                                .write(true)
                                .create(true)
                                .open(&destination_file_path)
                            {
                                Ok(f) => f,
                                Err(_err) => {
                                    println!("Error while writing at {:?}", "hee");
                                    exit(1);
                                }
                            };

                            match new_file.write_all(contents.as_bytes()) {
                                Ok(_) => println!(
                                    "Created file {}",
                                    &destination_file_path.to_str().unwrap()
                                ),
                                Err(_err) => {
                                    println!("Error while writing at {:?}", destination_file_path);
                                    exit(1);
                                }
                            }
                        } else if file.file_type().unwrap().is_dir() {
                            mkdir(format!(
                                "{}/{}",
                                destination.to_str().unwrap(),
                                file.file_name().into_string().unwrap()
                            ));

                            let mut new_source = source_dir.clone();
                            new_source.push(file.file_name());

                            let mut new_destination = destination.clone();
                            new_destination.push(file.file_name());
                            create_dir_files(new_source, new_destination)
                        }
                    }
                    Err(err) => {
                        eprintln!("Error while reading directory or file");
                        eprintln!("{}", err);
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("error occured while reading files from source");
            eprintln!("{}", err);
            exit(1);
        }
    }
}

pub fn mkdir(dir_name: String) {
    match fs::create_dir(&dir_name) {
        Ok(_) => {
            println!("Created dir {}", dir_name)
        }
        Err(_err) => {
            eprintln!("Error while creating dir {}", dir_name);
            exit(1);
        }
    }
}
