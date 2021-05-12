extern crate byte_unit;
//use byte_unit::Byte;
extern crate clap;
use clap::{Arg, App};
use std::fs::OpenOptions;
use std::io;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Seek;
use std::io::SeekFrom;
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
        .arg(Arg::with_name("iseek")
            .help("skip N ibs-sized blocks at start of output")
            .long("iseek")
            .alias("iskip")
            .takes_value(true)
            .value_name("N")
        )
        .arg(Arg::with_name("oseek")
            .help("skip N obs-sized blocks at start of output")
            .long("oseek")
            .alias("oskip")
            .takes_value(true)
            .value_name("N")
        )
        .arg(Arg::with_name("append")
            .help("append to the end of the output file (implies notruncate)")
            .long("append")
            .takes_value(false)
        )
        .arg(Arg::with_name("notrunc")
            .help("don't truncate the output file")
            .long("notruncate")
            .alias("notrunc")
            .takes_value(false)
        )
        .get_matches();

    let infile_name = matches.value_of("INPUT").unwrap();
    let outfile_name = matches.value_of("OUTPUT").unwrap();

    let trunc = !(matches.is_present("notrunc") || matches.is_present("append"));
    let append = matches.is_present("append");

    let ibs = match matches.value_of("ibs") {
        None => None,
        Some(ibs_str) => {
            match ibs_str.parse::<usize>() {
                Err(e) => panic!(
                    "ERROR: Can't parse ibs argument '{}': {}",
                    ibs_str,
                    e),
                Ok(ibs) => Some(ibs)
            }
        }
    };
    let ibs = ibs.unwrap_or(BLOCKSIZE);

    let mut infile = OpenOptions::new()
        .read(true)
        .open(infile_name)?;

    let iseek = matches.value_of("iseek")
        .unwrap_or("0")
        .parse::<u64>()
        .unwrap_or(0);
    let iseek = iseek * ibs as u64;

    infile.seek(SeekFrom::Start(iseek)).unwrap();

    let obs = matches.value_of("obs");
    let obs = match obs {
        None => BLOCKSIZE,
        Some(obs) => obs.parse::<usize>().unwrap_or(BLOCKSIZE)
    };

    let mut outfile = OpenOptions::new()
        .write(true)
        .truncate(trunc)
        .append(append)
        .open(outfile_name)?;

    let oseek = matches.value_of("oseek")
        .unwrap_or("0")
        .parse::<u64>()
        .unwrap_or(0);
    let oseek = oseek * obs as u64;
    outfile.seek(SeekFrom::Start(oseek)).unwrap();

    let mut reader = BufReader::with_capacity(ibs, infile);
    let mut writer = BufWriter::with_capacity(obs, outfile);

    io::copy(&mut reader, &mut writer)?;

    Ok(())
}
