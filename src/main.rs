use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::io::BufWriter;
use std::time::Instant;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "compressor_decompressor",
    about = "A simple file compressor and decompressor."
)]
enum Opt {
    Compress {
        #[structopt(parse(from_os_str))]
        input: std::path::PathBuf,
        #[structopt(parse(from_os_str))]
        output: std::path::PathBuf,
    },
    Decompress {
        #[structopt(parse(from_os_str))]
        input: std::path::PathBuf,
        #[structopt(parse(from_os_str))]
        output: std::path::PathBuf,
    },
}

fn main() {
    let opt = Opt::from_args();
    match opt {
        Opt::Compress { input, output } => {
            let mut input = BufReader::new(File::open(input).unwrap());
            let output = File::create(output).unwrap();
            let mut encoder = GzEncoder::new(output, Compression::default());
            let start = Instant::now();
            copy(&mut input, &mut encoder).unwrap();
            let output = encoder.finish().unwrap();
            println!(
                "Input size: {} bytes",
                input.get_ref().metadata().unwrap().len()
            );
            println!("Output size: {} bytes", output.metadata().unwrap().len());
            println!("Elapsed: {} ms", start.elapsed().as_millis());
        }
        Opt::Decompress { input, output } => {
            let input = BufReader::new(File::open(input).unwrap());
            let mut decoder = GzDecoder::new(input);
            let mut output = BufWriter::new(File::create(output).unwrap());
            let start = Instant::now();
            copy(&mut decoder, &mut output).unwrap();

            println!("Elapsed: {} ms", start.elapsed().as_millis());
        }
    }
}
