use std::fs;

use structopt::StructOpt;

use modeling::{by_dir, ParseOption};
use modeling::file_filter::FileFilter;
use modeling::render::{MermaidRender, PlantUmlRender};
use ignore::{WalkBuilder, DirEntry, Error};

#[derive(StructOpt, Debug, PartialEq, Clone)]
#[structopt(name = "basic")]
struct Opts {
    /// merge for same method name
    #[structopt(short, long)]
    merge: bool,

    /// multiple modules
    #[structopt(short, long)]
    by_modules: bool,

    /// input dir
    #[structopt(short, long, default_value = ".")]
    input: String,

    /// mermaid or puml
    #[structopt(short, long, default_value = "puml")]
    output_type: String,

    // filter dir
    #[structopt(long, short, use_delimiter = true)]
    packages: Vec<String>,

    // filter suffixes
    #[structopt(long, short, use_delimiter = true)]
    suffixes: Vec<String>,
}

impl Opts {
    pub fn to_parse_option(&self) -> ParseOption {
        ParseOption {
            merge: self.merge
        }
    }
}

fn main() {
    let opts: Opts = Opts::from_args();

    println!("Input path: {:?}", opts.input);
    println!("packages: {:?}", opts.packages);
    println!("suffixes: {:?}", opts.suffixes);

    let parse_option = opts.to_parse_option();
    let filter = FileFilter::new(opts.packages.clone(), opts.suffixes.clone());

    if !opts.by_modules {
        output_all_in_one(opts, parse_option, filter);
        return;
    }

    // for result in WalkBuilder::new("./").max_depth(Some(1)).build() {
    //     match result {
    //         Ok(dir) => {
    //           by_dir()
    //         }
    //         Err(_) => {}
    //     }
    // }

}

fn output_all_in_one(opts: Opts, parse_option: ParseOption, filter: FileFilter) {
    let classes = by_dir(&opts.input, filter, parse_option);

    match opts.output_type.as_str() {
        "mermaid" => {
            let uml = MermaidRender::render(&classes);
            let _ = fs::write("modeling.mermaid", uml);
        }
        &_ => {
            let uml = PlantUmlRender::render(&classes);
            let _ = fs::write("modeling.puml", uml);
        }
    }
}
