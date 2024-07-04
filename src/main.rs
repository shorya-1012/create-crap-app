mod create_files;

use std::{
    env, fs,
    path::{Path, PathBuf},
    process::exit,
};

use create_files::{create_dir_files, mkdir};
use inquire::{validator::Validation, Select, Text};

#[derive(Debug)]
struct UserInput {
    project_name: String,
    template_choice: String,
}

fn main() {
    println!("Welcome to create crap app");
    let user_input = get_user_input();

    let mut templates_source = get_source_dir();
    templates_source.push(user_input.template_choice);

    mkdir(format!(
        "{}/{}",
        get_curr_dir().to_str().unwrap(),
        user_input.project_name
    ));

    let mut destination = get_curr_dir();
    destination.push(user_input.project_name);

    create_dir_files(templates_source, destination);
}

fn get_source_dir() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let templates_path = Path::new(manifest_dir).join("templates");
    templates_path
}

fn get_curr_dir() -> PathBuf {
    env::current_dir().unwrap()
}

fn get_user_input() -> UserInput {
    let project_name_validator = |input: &str| {
        if input.chars().count() > 40 {
            Ok(Validation::Invalid(
                "You're only allowed 40 characters.".into(),
            ))
        } else if input.chars().count() < 3 {
            Ok(Validation::Invalid(
                "Project name should contain atlead 3 charaters".into(),
            ))
        } else {
            Ok(Validation::Valid)
        }
    };

    let project_name = match Text::new("Enter project name : ")
        .with_validator(project_name_validator)
        .prompt()
    {
        Ok(name) => name,
        Err(_err) => {
            eprintln!("Unable to read input");
            exit(1);
        }
    };

    let template_choice = match Select::new(
        "How would you like to use react : ",
        get_project_templates(),
    )
    .prompt()
    {
        Ok(choice) => choice,
        Err(_err) => {
            eprintln!("Unable to read input");
            exit(1);
        }
    };

    UserInput {
        project_name,
        template_choice,
    }
}

fn get_project_templates() -> Vec<String> {
    let mut options: Vec<String> = Vec::new();

    let source_dir_path = get_source_dir();

    match fs::read_dir(source_dir_path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(file) => {
                        options.push(file.file_name().into_string().unwrap());
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

    options
}
