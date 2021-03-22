extern crate clap;
use clap::{Arg, ArgGroup, App};
use std::fs::OpenOptions;
use std::io;
use std::io::BufReader;
use std::io::BufWriter;

fn main() -> io::Result<()> {
    let matches = App::new("rusdd: dd in Rust")
        .version(clap::crate_version!())
        .about("Philipp Rosenberger <rusdd@iluminat23.org>")
        .arg(Arg::with_name("INPUT")
            .help("Input file")
            .required(true)
            .index(1)
        )
        .arg(Arg::with_name("OUTPUT")
            .help("Output file")
            .required(true)
            .index(2)
        )
        .args_from_usage(
            "--bs [blocksize] 'set the input and output blocksize'
                                        --ibs [blocksize]   'set the input blocksize'
                                        --obs [blocksize]   'set the output blocksize",
        )
        .group(
            ArgGroup::with_name("blocksize")
            .args(&["bs", "ibs", "obs"]),
        )
        .get_matches();

    let infile_name = matches.value_of("INPUT").unwrap();
    let outfile_name = matches.value_of("OUTPUT").unwrap();

    let infile = OpenOptions::new()
        .read(true)
        .open(infile_name)?;
    let outfile = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(outfile_name)?;

    let buf_size = 1 * 1024 * 1024;
    let mut reader = BufReader::with_capacity(buf_size, infile);
    let mut writer = BufWriter::with_capacity(buf_size, outfile);

    io::copy(&mut reader, &mut writer)?;

    Ok(())
}
