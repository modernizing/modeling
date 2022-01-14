use std::collections::HashMap;
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

    for result in WalkBuilder::new(opts.input).max_depth(Some(1)).build() {
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
    let mut map: HashMap<String, u32> = HashMap::default();
    let classes = by_dir(dir.path(), filter.clone(), parse_option);
    for class in classes {
        let counter = map.entry(class.name).or_insert(0);
        *counter += 1;

        for method in class.methods {
            let counter = map.entry(method.name).or_insert(0);
            *counter += 1;
        }

        for member in class.members {
            let counter = map.entry(member.name).or_insert(0);
            *counter += 1;
        }
    }

    let mut hash_vec: Vec<(&String, &u32)> = map.iter().collect();
    hash_vec.sort_by(|a, b| b.1.cmp(a.1));

    println!("Sorted: {:?}", hash_vec);
}