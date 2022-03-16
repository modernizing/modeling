#[macro_use]
extern crate prettytable;

use log::info;
use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};

use prettytable::{format, row, Table};
use structopt::StructOpt;

use modeling::file_filter::FileFilter;
use modeling::segment::segment;
use modeling::segment::stop_words::{STOP_WORDS, TECH_STOP_WORDS};
use modeling::{by_dir, ClassInfo, ParseOption};

#[derive(StructOpt, Debug, PartialEq, Clone)]
#[structopt(name = "basic")]
struct ConceptOpts {
    /// input dir
    #[structopt(short, long, default_value = ".")]
    input: String,
    /// filter by packages, like: `com.phodal.modeling`
    #[structopt(long, short, use_delimiter = true)]
    packages: Vec<String>,
    /// filter by suffixes, like: `java` for .java file
    #[structopt(long, short, use_delimiter = true)]
    suffixes: Vec<String>,
    /// by grep regex rules: for example: `.*Service`
    #[structopt(short, long, default_value = "")]
    grep: String,
}

impl ConceptOpts {
    pub fn to_parse_option(&self) -> ParseOption {
        ParseOption {
            merge_method_name: false,
            field_only: false,
            without_parent: false,
            without_impl_suffix: false,
            inline_id_suffix: false,
            without_suffix: "".to_string()
        }
    }
}

fn main() {
    env_logger::init();
    let opts: ConceptOpts = ConceptOpts::from_args();

    info!("parse input {:?} with {:?}", &opts.input, &opts);

    let parse_option = opts.to_parse_option();
    let filter = FileFilter::new(
        opts.packages.clone(),
        opts.suffixes.clone(),
        opts.grep.clone(),
    );

    output_by_dir(&parse_option, &filter, &PathBuf::from(&opts.input));
}

fn output_by_dir(parse_option: &ParseOption, filter: &FileFilter, dir: &Path) {
    let classes = by_dir(dir, filter.clone(), parse_option);
    let (word, text) = class_to_identify_map(&classes);

    map_to_csv(word, "output_word.csv");
    map_to_csv(text, "output_text.csv");
}

fn map_to_csv(word: HashMap<String, u32>, path: &str) {
    let mut hash_vec: Vec<(&String, &u32)> = word.iter().collect();
    hash_vec.sort_by(|a, b| b.1.cmp(a.1));

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    for (key, value) in hash_vec {
        table.add_row(row![key, value.to_string()]);
    }

    let out = File::create(path).unwrap();
    table.to_csv(out).unwrap();
}

fn class_to_identify_map(classes: &Vec<ClassInfo>) -> (HashMap<String, u32>, HashMap<String, u32>) {
    let mut by_word: HashMap<String, u32> = HashMap::default();
    let mut by_text: HashMap<String, u32> = HashMap::default();
    info!("class counts: {:?}", &classes.len());

    let mut methods_counts = 0;
    for class in classes {
        count_words(&mut by_word, &class.name);
        count_text(&mut by_text, &class.name);

        methods_counts = methods_counts + class.methods.len();
        for method in &class.methods {
            count_words(&mut by_word, &method.name);
            count_text(&mut by_text, &method.name);
        }

        for member in &class.members {
            count_words(&mut by_word, &member.name);
            count_text(&mut by_text, &member.name);
        }
    }

    info!("methods counts: {:?}", methods_counts);
    (by_word, by_text)
}

fn count_text(map: &mut HashMap<String, u32>, var: &str) {
    let counter = map.entry(var.to_string()).or_insert(0);
    *counter += 1;
}

fn count_words(map: &mut HashMap<String, u32>, var: &str) {
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
