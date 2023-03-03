use std::fs;

use dialoguer::{Confirm, Select};
use miette::{miette, IntoDiagnostic, Result};

static PROJECT_DIRS: [&str; 5] = [
    "0 Inbox",
    "1 Projects",
    "2 Areas",
    "3 References",
    "4 Archive",
];

fn create_directories() -> Result<()> {
    println!("Creating Directories");

    let old_files = fs::read_dir(".").into_diagnostic()?;

    for dir in PROJECT_DIRS {
        fs::create_dir(dir).map_err(|_| miette!("Could not create directory '{}'", dir))?;
    }

    for old_file in old_files {
        let old_file = old_file.into_diagnostic()?;
        fs::rename(
            old_file.path(),
            "4 Archive/".to_owned() + old_file.file_name().to_str().unwrap(),
        ).into_diagnostic()?;
    }

    Ok(())
}

fn remove_directories() -> Result<()> {
    if !Confirm::new()
        .with_prompt("Do you want to delete the Second Brain Directories?")
        .default(false)
        .interact()
        .into_diagnostic()?
    {
        return Ok(());
    }

    println!("Removing Directories");
    for dir in PROJECT_DIRS {
        fs::remove_dir(dir).map_err(|_| miette!("Could not remove directory '{}'", dir))?;
    }

    Ok(())
}

fn main() -> Result<()> {
    let functions = ["create", "remove"];
    let selection = Select::new()
        .with_prompt("Please choose whether you want to create or remove Second Brain directories")
        .items(&functions)
        .default(0)
        .interact()
        .into_diagnostic()?;

    match selection {
        0 => create_directories()?,
        1 => remove_directories()?,
        _ => {}
    }

    println!("Done!");

    Ok(())
}
