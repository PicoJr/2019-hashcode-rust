extern crate rayon;

use rayon::prelude::*;
use structopt::StructOpt;

use hashcode2019::{cli, dump, parse_input_file, solve};

fn main() -> std::io::Result<()> {
    let args = cli::Cli::from_args();
    let images = parse_input_file(args.path);
    let result = images.par_chunks(args.chunk_size).map(|chunk| solve(chunk));
    let result = result.flatten();
    let slides = result.collect();
    let mut output_path = std::path::PathBuf::new();
    output_path.push("./out.txt");
    dump(output_path, slides);
    Ok(())
}