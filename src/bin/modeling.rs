use std::fs;

use clap::{AppSettings, Clap};
use structopt::StructOpt;

use modeling::by_dir;
use modeling::render::{MermaidRender, PlantUmlRender};

#[derive(StructOpt, Debug, Clap)]
#[clap(version = "1.0", author = "Inherd Group <group@inherd.org>")]
#[clap(setting = AppSettings::ColoredHelp)]
#[structopt(name = "basic")]
struct Opts {
    /// input dir
    #[clap(short, long, default_value = ".")]
    input: String,

    /// mermaid or puml
    #[clap(short, long, default_value = "puml")]
    output_type: String,

    // filter dir
    #[clap(long, short, use_delimiter = true)]
    packages: Vec<String>,

    // filter suffixes
    #[clap(long, short, use_delimiter = true)]
    suffixes: Vec<String>,
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("Input path: {:?}", opts.input);
    println!("packages: {:?}", opts.packages);
    println!("suffixes: {:?}", opts.suffixes);

    let classes = by_dir(opts.input);

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
