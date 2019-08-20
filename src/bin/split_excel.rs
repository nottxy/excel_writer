use std::{
    env,
    fs::{self, File, OpenOptions},
    io::{self, Result},
    path::Path,
};

use zip::ZipArchive;

fn main() -> Result<()> {
    let (to_dir, from_excel_file) = read_args(env::args());

    println!(
        "begin to split, to_dir: {}, from_excel_file: {}",
        &to_dir, &from_excel_file
    );

    split_excel(&to_dir, &from_excel_file)?;

    println!("split done");

    Ok(())
}

fn read_args(args: env::Args) -> (String, String) {
    let mut args = args.skip(1);

    let to_dir = args.next().unwrap_or_else(|| "data".to_string());
    let from_excel_file = args.next().unwrap_or_else(|| "template.xlsx".to_string());

    (to_dir, from_excel_file)
}

fn split_excel(to_dir: &str, from_excel_file: &str) -> Result<()> {
    let mut zip_archive = ZipArchive::new(File::open(from_excel_file)?)?;

    for i in 0..zip_archive.len() {
        let mut zip_file = zip_archive.by_index(i)?;

        let mut to_file = reset_file(to_dir, zip_file.name())?;

        io::copy(&mut zip_file, &mut to_file)?;
    }

    Ok(())
}

fn reset_file(to_dir: &str, file_name: &str) -> Result<File> {
    let file_path = format!("{}/{}", to_dir, file_name);
    let file_path = Path::new(&file_path);

    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent)?;
    }

    OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)
}
