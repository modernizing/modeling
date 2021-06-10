use std::fs;

use clap::{AppSettings, Clap};

use modeling::by_dir;
use modeling::render::{MermaidRender, PlantUmlRender};

#[derive(Clap)]
#[clap(version = "1.0", author = "Inherd Group <group@inherd.org>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(short, long, default_value = ".")]
    source_dir: String,

    #[clap(short, long, default_value = "puml")]
    output_type: String,

    #[clap(long)]
    packages: Vec<String>,

    #[clap(long)]
    suffixes: Vec<String>,
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("Input path: {:?}", opts.source_dir);

    let classes = by_dir(opts.source_dir);

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
