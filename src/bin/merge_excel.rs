use std::{
    env,
    fs::{self, File, OpenOptions},
    io::{self, Cursor, Result, Seek, Write},
    path::Path,
};

use zip::{write::FileOptions, CompressionMethod, ZipWriter};

fn main() -> Result<()> {
    let (from_dir, to_excel_file) = read_args(env::args());

    println!(
        "begin to merge, from_dir: {}, to_excel_file: {}",
        &from_dir, &to_excel_file
    );

    merge_excel(&from_dir, &to_excel_file)?;

    println!("merge done");

    Ok(())
}

fn merge_excel(from_dir: &str, to_excel_file: &str) -> Result<()> {
    let mut zip = ZipWriter::new(Cursor::new(Vec::default()));

    let from_dir = Path::new(from_dir);

    write_dir(from_dir, &mut zip)?;

    let mut buffer = Cursor::new(zip.finish()?.into_inner());

    let mut to_excel_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(to_excel_file)?;

    io::copy(&mut buffer, &mut to_excel_file).map(|_| ())
}

fn write_dir<W: Write + Seek>(dir: &Path, zip: &mut ZipWriter<W>) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            write_dir(&path, zip)?;
        } else {
            add_zip_file(&path, zip)?;
        }
    }
    Ok(())
}

fn add_zip_file<W: Write + Seek>(file_path: &Path, zip: &mut ZipWriter<W>) -> Result<()> {
    let mut file = File::open(file_path)?;

    let excel_file_path = {
        let mut file_path_iter = file_path.iter();
        file_path_iter.next();
        file_path_iter.as_path()
    };

    let options = FileOptions::default().compression_method(CompressionMethod::Deflated);
    zip.start_file(excel_file_path.to_string_lossy(), options)?;
    io::copy(&mut file, zip).map(|_| ())
}

fn read_args(args: env::Args) -> (String, String) {
    let mut args = args.skip(1);

    let from_dir = args.next().unwrap_or_else(|| "data".to_string());
    let to_excel_file = args.next().unwrap_or_else(|| "merged.xlsx".to_string());

    (from_dir, to_excel_file)
}
