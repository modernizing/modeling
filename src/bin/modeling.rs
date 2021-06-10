use std::fs;

use structopt::StructOpt;

use modeling::by_dir;
use modeling::file_filter::FileFilter;
use modeling::render::{MermaidRender, PlantUmlRender};

#[derive(StructOpt, Debug, PartialEq)]
#[structopt(name = "basic")]
struct Opts {
    /// merge for same method name
    #[structopt(short, long)]
    merge: bool,

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

fn main() {
    let opts: Opts = Opts::from_args();

    println!("Input path: {:?}", opts.input);
    println!("packages: {:?}", opts.packages);
    println!("suffixes: {:?}", opts.suffixes);

    let filter = FileFilter::new(opts.packages, opts.suffixes);
    let classes = by_dir(opts.input, filter);

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
