#[macro_use]
extern crate lazy_static;
extern crate serde;

use ignore::Walk;

use structopt::StructOpt;

pub use coco_struct::{ClassInfo, MemberInfo, MethodInfo};
pub use ctags::ctags_cmd::CmdCtags;
pub use ctags::ctags_opt::Opt;
pub use ctags::ctags_parser::CtagsParser;
use std::path::Path;

pub mod ctags;
pub mod render;
pub mod coco_struct;

/// Returns Vec<ClassInfo> with the given path.
///
/// # Arguments
///
/// * `path` - CODE PATH
///
/// # Examples
///
/// ```
/// use modeling::{by_dir};
/// use modeling::render::PlantUmlRender;
///
/// let classes = by_dir("src/");
/// let puml = PlantUmlRender::render(&classes);
/// ```
pub fn by_dir<P: AsRef<Path>>(path: P) -> Vec<ClassInfo> {
    let origin_files = files_from_path(path);

    by_files(origin_files)
}

/// Returns Vec<ClassInfo> with the given files.
///
/// # Arguments
///
/// * `files` - code files in string
///
/// # Examples
///
/// ```
/// use modeling::{by_files};
/// use modeling::render::PlantUmlRender;
///
/// let mut files = vec![];
/// files.push("src/lib.rs".to_string());
/// let classes = by_files(files);
/// let puml = PlantUmlRender::render(&classes);
/// ```
pub fn by_files(files: Vec<String>) -> Vec<ClassInfo> {
    let thread = count_thread(&files);
    let opt = build_opt(thread);

    run_ctags(&opt, &files_by_thread(files, &opt))
}

fn count_thread(origin_files: &Vec<String>) -> usize {
    let mut thread = origin_files.len();
    let default_ptags_thread = 8;
    if thread >= default_ptags_thread {
        thread = default_ptags_thread;
    }
    thread
}

fn run_ctags(opt: &Opt, files: &Vec<String>) -> Vec<ClassInfo> {
    let outputs = CmdCtags::call(&opt, &files).unwrap();
    let mut iters = Vec::new();
    for o in &outputs {
        let iter = if opt.validate_utf8 {
            std::str::from_utf8(&o.stdout).unwrap().lines()
        } else {
            unsafe { std::str::from_utf8_unchecked(&o.stdout).lines() }
        };
        iters.push(iter);
    }

    let parser = CtagsParser::parse_str(iters);
    let classes = parser.classes();

    classes
}

fn files_from_path<P: AsRef<Path>>(path: P) -> Vec<String> {
    let mut origin_files = vec![];
    for result in Walk::new(path) {
        if let Ok(entry) = result {
            if entry.file_type().unwrap().is_file() {
                origin_files.push(format!("{}", entry.path().display()))
            }
        }
    }
    origin_files
}

fn files_by_thread(origin_files: Vec<String>, opt: &Opt) -> Vec<String> {
    let mut files = vec![String::from(""); opt.thread];
    for (i, f) in origin_files.iter().enumerate() {
        files[i % opt.thread].push_str(f);
        files[i % opt.thread].push_str("\n");
    }
    files
}

fn build_opt(thread: usize) -> Opt {
    let string = thread.to_string();
    let thread: &str = string.as_str();
    let args = vec!["ptags", "-t", thread, "--verbose=true", "--fields=+latinK"];
    let opt = Opt::from_iter(args.iter());
    opt
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::fs;
    use crate::render::{MermaidRender, PlantUmlRender};
    use crate::by_dir;

    pub fn ctags_fixtures_dir() -> PathBuf {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let ctags_dir = root_dir.join("_fixtures")
            .join("ctags")
            .join("source")
            .join("animal.ts");

        return ctags_dir;
    }

    #[test]
    fn should_run_struct_analysis() {
        let path = format!("{}", ctags_fixtures_dir().display());
        let vec = by_dir(path);

        assert_eq!(3, vec.len());
        let result = PlantUmlRender::render(&vec);

        let _ = fs::write("demo.puml", result.clone());
        assert!(result.contains("class Animal"));
        assert!(result.contains("class Horse extends Animal"));
        assert!(result.contains("class Snake extends Animal "));
    }

    #[test]
    fn should_render_mermaid() {
        let path = format!("{}", ctags_fixtures_dir().display());
        let vec = by_dir(path);

        assert_eq!(3, vec.len());
        let result = MermaidRender::render(&vec);

        println!("{:?}", result);

        let _ = fs::write("demo.puml", result.clone());
        assert!(result.contains("class Animal"));
        assert!(result.contains("Animal <|-- Horse"));
        assert!(result.contains("Animal <|-- Snake"));
    }
}