#[macro_use]
extern crate prettytable;

use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};

use prettytable::{format, row, Table};
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

    let parse_option = opts.to_parse_option();
    let filter = FileFilter::new(opts.packages.clone(), opts.suffixes.clone(), opts.grep.clone());

    output_by_dir(&parse_option, &filter, &PathBuf::from(&opts.input));
}

fn output_by_dir(parse_option: &ParseOption, filter: &FileFilter, dir: &Path) {
    let mut map: HashMap<String, u32> = HashMap::default();
    let classes = by_dir(dir, filter.clone(), parse_option);
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

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    for (key, value) in hash_vec {
        table.add_row(row![key, value.to_string()]);
    }

    table.printstd();

    let out = File::create("output.csv").unwrap();
    table.to_csv(out).unwrap();
}