extern crate byte_unit;
//use byte_unit::Byte;
extern crate clap;
use clap::{Arg, App};
use std::fs::OpenOptions;
use std::io;
use std::io::BufReader;
use std::io::BufWriter;
//use std::convert::{From, TryFrom};

const BLOCKSIZE:usize = 4096;

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
        .arg(Arg::with_name("bs")
            .help("set the blocksize for input and output")
            .short("B")
            .long("bs")
            .alias("blocksize")
            .takes_value(true)
            .value_name("BLOCKSIZE")
        )
        .arg(Arg::with_name("ibs")
            .help("set the input blocksize")
            .short("I")
            .long("ibs")
            .alias("input-blocksize")
            .takes_value(true)
            .value_name("BLOCKSIZE")
            .conflicts_with("blocksize")
        )
        .arg(Arg::with_name("obs")
            .help("set the output blocksize")
            .short("O")
            .long("obs")
            .alias("outout-blocksize")
            .takes_value(true)
            .value_name("BLOCKSIZE")
            .conflicts_with("blocksize")
        )
        .get_matches();

    let infile_name = matches.value_of("INPUT").unwrap();
    let outfile_name = matches.value_of("OUTPUT").unwrap();

    let ibs = match matches.value_of("ibs") {
        None => BLOCKSIZE,
        Some(ibs) => match ibs.parse::<usize>() {
            Err(e) => panic!("ERROR: Can't parse ibs argument: {}", e),
            Ok(ibs) => ibs
        },
    };

    println!("ibs: {}", ibs);

    let infile = OpenOptions::new()
        .read(true)
        .open(infile_name)?;
    let outfile = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(outfile_name)?;

    let buf_size = 1 * 1024 * 1024;
    let mut reader = BufReader::with_capacity(ibs, infile);
    let mut writer = BufWriter::with_capacity(buf_size, outfile);

    io::copy(&mut reader, &mut writer)?;

    Ok(())
}
