use ignore::{DirEntry, WalkBuilder};
use structopt::StructOpt;

use modeling::{by_dir, ParseOption};
use modeling::file_filter::FileFilter;

#[derive(StructOpt, Debug, PartialEq, Clone)]
#[structopt(name = "basic")]
struct ConceptOpts {
    /// input dir
    #[structopt(short, long, default_value = ".")]
    input: String,

    // filter dir
    #[structopt(long, short, use_delimiter = true)]
    packages: Vec<String>,

    // filter suffixes
    #[structopt(long, short, use_delimiter = true)]
    suffixes: Vec<String>,

    #[structopt(short, long, default_value = "")]
    grep: String,
}


impl ConceptOpts {
    pub fn to_parse_option(&self) -> ParseOption {
        ParseOption {
            merge: false,
            field_only: false
        }
    }
}

fn main() {
    // 1. read words by class-words for map
    // 2. output words relationship
    // 3. count word frequency
    let opts: ConceptOpts = ConceptOpts::from_args();

    println!("Input path: {:?}", opts.input);
    println!("packages: {:?}", opts.packages);
    println!("suffixes: {:?}", opts.suffixes);

    let parse_option = opts.to_parse_option();
    let filter = FileFilter::new(opts.packages.clone(), opts.suffixes.clone(), opts.grep.clone());

    for result in WalkBuilder::new("./").max_depth(Some(1)).build() {
        if let Ok(dir) = result {
            let path = dir.path();
            if path.is_dir() {
                if let Some(_x) = path.file_name() {
                    output_by_dir(&parse_option, &filter, &dir)
                };
            }
        }
    }
}

fn output_by_dir(parse_option: &ParseOption, filter: &FileFilter, dir: &DirEntry) {
    let classes = by_dir(dir.path(), filter.clone(), parse_option);
    if classes.len() > 0 {
        println!("{:?}", classes);
    }
}