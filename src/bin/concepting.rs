#[macro_use]
extern crate prettytable;

use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};
use log::info;

use prettytable::{format, row, Table};
use structopt::StructOpt;

use modeling::{by_dir, ClassInfo, ParseOption};
use modeling::file_filter::FileFilter;
use modeling::segment::segment;
use modeling::segment::stop_words::{STOP_WORDS, TECH_STOP_WORDS};

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
    env_logger::init();
    let opts: ConceptOpts = ConceptOpts::from_args();

    info!("parse input {:?} with {:?}", &opts.input, &opts);

    let parse_option = opts.to_parse_option();
    let filter = FileFilter::new(opts.packages.clone(), opts.suffixes.clone(), opts.grep.clone());

    output_by_dir(&parse_option, &filter, &PathBuf::from(&opts.input));
}

fn output_by_dir(parse_option: &ParseOption, filter: &FileFilter, dir: &Path) {
    let classes = by_dir(dir, filter.clone(), parse_option);

    let map = class_to_identify_map(classes);

    let mut hash_vec: Vec<(&String, &u32)> = map.iter().collect();
    hash_vec.sort_by(|a, b| b.1.cmp(a.1));

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    for (key, value) in hash_vec {
        table.add_row(row![key, value.to_string()]);
    }
    //

    let out = File::create("output.csv").unwrap();
    table.to_csv(out).unwrap();
}

fn class_to_identify_map(classes: Vec<ClassInfo>) -> HashMap<String, u32> {
    let mut map: HashMap<String, u32> = HashMap::default();
    info!("class counts: {:?}", &classes.len());

    let mut methods_counts = 0;
    for class in &classes {
        count_it(&mut map, &class.name);

        methods_counts = methods_counts + class.methods.len();
        for method in class.methods {
            count_it(&mut map, &method.name);
        }

        for member in class.members {
            count_it(&mut map, &member.name);
        }
    }

    info!("methods counts: {:?}", methods_counts);
    map
}

fn count_it(map: &mut HashMap<String, u32>, var: &str) {
    for word in segment(var) {
        if STOP_WORDS.contains(&&**&word.to_lowercase()) {
            continue;
        }

        if TECH_STOP_WORDS.contains(&&**&word.to_lowercase()) {
            continue;
        }

        let counter = map.entry(word).or_insert(0);
        *counter += 1;
    }
}