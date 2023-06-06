use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Result};
use std::path::Path;

pub fn open_buffered_read<P>(options: &mut OpenOptions, path: P) -> Result<BufReader<File>>
where
    P: AsRef<Path>,
{
    let f = options.read(true).open(path.as_ref())?;
    let f = BufReader::new(f);

    Ok(f)
}

pub fn open_buffered_write<P>(options: &mut OpenOptions, path: P) -> Result<BufWriter<File>>
where
    P: AsRef<Path>,
{
    let f = options.write(true).open(path.as_ref())?;
    let f = BufWriter::new(f);

    Ok(f)
}
