use std::fs;

use ignore::{WalkBuilder, DirEntry};
use structopt::StructOpt;

use modeling::{by_dir, ClassInfo, ParseOption};
use modeling::file_filter::FileFilter;
use modeling::render::{MermaidRender, PlantUmlRender};
use std::ffi::OsStr;
use modeling::render::graphviz_render::GraphvizRender;

#[derive(StructOpt, Debug, PartialEq, Clone)]
#[structopt(name = "Modeling")]
struct Opts {
    /// output debug information
    #[structopt(short, long)]
    debug: bool,
    /// merge for same method name
    #[structopt(short, long)]
    merge: bool,
    /// multiple modules
    #[structopt(short, long)]
    by_modules: bool,
    /// input dir
    #[structopt(short, long, default_value = ".")]
    input: String,
    /// support: puml, mermaid, graphviz with json
    #[structopt(short, long, default_value = "puml")]
    output_type: String,
    /// filter by packages, like: `com.phodal.modeling`
    #[structopt(long, short, use_delimiter = true)]
    packages: Vec<String>,
    /// filter by suffixes, like: `java` for .java file
    #[structopt(long, short, use_delimiter = true)]
    suffixes: Vec<String>,
    /// only load field in methods
    #[structopt(short, long)]
    field_only: bool,
    /// by grep regex rules: for example: `.*Service`
    #[structopt(short, long, default_value = "")]
    grep: String,
    /// without class inheritance
    #[structopt(long)]
    without_parent: bool,
    /// if class's prop start with `IRepository` will become `Repository`
    #[structopt(long)]
    without_impl_suffix: bool,
    /// if class's prop end with Id and class in list, will replace `int` type to `xxClass`
    #[structopt(long)]
    inline_id_suffix: bool,
}

impl Opts {
    pub fn to_parse_option(&self) -> ParseOption {
        ParseOption {
            merge: self.merge,
            field_only: self.field_only,
            without_parent: self.without_parent,
            without_impl_suffix: self.without_impl_suffix,
            inline_id_suffix: self.inline_id_suffix,
        }
    }
}

fn main() {
    let opts: Opts = Opts::from_args();

    let parse_option = opts.to_parse_option();
    let filter = FileFilter::new(opts.packages.clone(), opts.suffixes.clone(), opts.grep.clone());

    if !opts.by_modules {
        output_all_in_one(opts, &parse_option, filter);
        return;
    }

    for result in WalkBuilder::new(&opts.input).max_depth(Some(1)).build() {
        if let Ok(dir) = result {
            let path = dir.path();
            if path.is_dir() {
                if let Some(x) = path.file_name() {
                    output_by_dir(&opts, &parse_option, &filter, &dir, x)
                };
            }
        }
    }
}

fn output_by_dir(opts: &Opts, parse_option: &ParseOption, filter: &FileFilter, dir: &DirEntry, x: &OsStr) {
    let dir_name = x.to_str().unwrap();
    let classes = by_dir(dir.path(), filter.clone(), parse_option);
    if classes.len() > 0 {
        output_file(&opts, &classes, dir_name)
    }
}

fn output_all_in_one(opts: Opts, parse_option: &ParseOption, filter: FileFilter) {
    let classes = by_dir(&opts.input, filter, parse_option);

    if opts.debug {
        let _ = fs::write("debug.json", serde_json::to_string(&classes).unwrap());
    }

    output_file(&opts, &classes, "modeling");
}

fn output_file(opts: &Opts, classes: &Vec<ClassInfo>, name: &str) {
    let parse_option = opts.to_parse_option();
    match opts.output_type.as_str() {
        "mermaid" => {
            let uml = MermaidRender::render(&classes, &parse_option);
            let file_name = format!("{}.mermaid", name);
            let _ = fs::write(file_name, uml);
        }
        "graphviz" => {
            let graph = GraphvizRender::render(&classes, &parse_option);
            let file_name = format!("{}.dot", name);
            let _ = fs::write(file_name, graph);
        }
        &_ => {
            let uml = PlantUmlRender::render(&classes, &parse_option);
            let file_name = format!("{}.puml", name);
            let _ = fs::write(file_name, uml);
        }
    }
}
