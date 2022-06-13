use std::fs::File;

use clap::Parser;
use zip::write::FileOptions;

/// Inflate files to a much more reasonable size to fill your disk
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Cli {
    /// Zip file to create
    out_file: String,
    /// Amount to mulitiply each file size by
    #[clap(long, short = 'm', default_value_t = 0.0)]
    multiplier: f64,
    /// Amount of bytes to add to the zip file
    #[clap(long, short = 'b', default_value_t = 0)]
    additional_bytes: u64,
    /// Files to inflate
    in_files: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    write_files(
        std::fs::File::create(cli.out_file).unwrap(),
        &cli.in_files,
        cli.additional_bytes,
        cli.multiplier,
    )
    .unwrap();
}

fn write_files(
    file: File,
    paths: &[String],
    additional_bytes: u64,
    mutliplier: f64,
) -> std::io::Result<()> {
    let mut zip = zip::ZipWriter::new(file);

    let options = FileOptions::default()
        .compression_level(Some(0))
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    let mut padded = additional_bytes == 0;

    for path in paths {
        zip.start_file(path, options)?;

        let mut in_file = std::fs::File::open(path)?;

        let file_size = std::io::copy(&mut in_file, &mut zip)?;

        let mut num_bytes_to_write = 0;

        if !padded {
            padded = true;
            num_bytes_to_write = additional_bytes / 5
        }

        if mutliplier != 0.0 {
            num_bytes_to_write = (file_size as f64 * mutliplier).ceil() as u64;
        }

        for _ in 0..num_bytes_to_write {
            zip.write_padding()?;
        }
    }
    zip.finish()?;
    Ok(())
}
