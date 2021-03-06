#[macro_use]
extern crate lazy_static;
extern crate serde;

use std::path::Path;

use ignore::Walk;
use structopt::StructOpt;

pub use coco_struct::{ClassInfo, MemberInfo, MethodInfo};
pub use ctags::ctags_cmd::CmdCtags;
pub use ctags::ctags_opt::Opt;
pub use ctags::ctags_parser::CtagsParser;
pub use file_filter::*;
pub use parse_option::ParseOption;

use crate::file_filter::FileFilter;

pub mod coco_struct;
pub mod ctags;
pub mod file_filter;
pub mod parse_option;
pub mod render;
pub mod segment;

/// Returns Vec<ClassInfo> with the given path.
///
/// # Arguments
///
/// * `path` - CODE PATH
///
/// # Examples
///
/// ```
/// use modeling::{by_dir, ParseOption};
/// use modeling::render::PlantUmlRender;
///
/// use modeling::file_filter::FileFilter;
/// let classes = by_dir("src/",FileFilter::default(), &ParseOption::default());
/// let puml = PlantUmlRender::render(&classes, &ParseOption::default());
/// ```
pub fn by_dir<P: AsRef<Path>>(path: P, filter: FileFilter, option: &ParseOption) -> Vec<ClassInfo> {
    by_files(files_from_path(path, filter), option)
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
/// use modeling::{by_files, ParseOption};
/// use modeling::render::PlantUmlRender;
///
/// let mut files = vec![];
/// files.push("src/lib.rs".to_string());
/// let classes = by_files(files, &ParseOption::default());
/// let puml = PlantUmlRender::render(&classes, &ParseOption::default());
/// ```
pub fn by_files(files: Vec<String>, option: &ParseOption) -> Vec<ClassInfo> {
    let thread = count_thread(&files);
    let opt = build_opt(thread);

    run_ctags(&opt, &files_by_thread(files, &opt), option)
}

fn count_thread(origin_files: &[String]) -> usize {
    let mut thread = origin_files.len();
    let default_ptags_thread = 8;
    if thread >= default_ptags_thread {
        thread = default_ptags_thread;
    }
    thread
}

fn run_ctags(opt: &Opt, files: &[String], option: &ParseOption) -> Vec<ClassInfo> {
    let outputs = CmdCtags::call(opt, files).unwrap();
    let mut iters = Vec::new();
    for o in &outputs {
        let iter = if opt.validate_utf8 {
            std::str::from_utf8(&o.stdout).unwrap().lines()
        } else {
            unsafe { std::str::from_utf8_unchecked(&o.stdout).lines() }
        };
        iters.push(iter);
    }

    let mut parser = CtagsParser::parse_str(iters);

    parser.option = option.clone();
    parser.classes()
}

fn files_from_path<P: AsRef<Path>>(path: P, filter: FileFilter) -> Vec<String> {
    let mut origin_files = vec![];
    for entry in Walk::new(path).flatten() {
        if entry.file_type().unwrap().is_file() {
            let buf = entry.path().to_path_buf();
            if filter.allow(buf) {
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
        files[i % opt.thread].push('\n');
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
    use std::fs;
    use std::path::PathBuf;

    use crate::file_filter::FileFilter;
    use crate::render::{MermaidRender, PlantUmlRender};
    use crate::{by_dir, ParseOption};

    pub fn ctags_fixtures_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("_fixtures")
            .join("ctags")
            .join("source")
            .join("animal.ts")
    }

    #[test]
    fn should_run_struct_analysis() {
        let path = format!("{}", ctags_fixtures_dir().display());
        let option = ParseOption::default();
        let vec = by_dir(path, FileFilter::default(), &option);

        assert_eq!(3, vec.len());
        let result = PlantUmlRender::render(&vec, &option);

        let _ = fs::write("demo.puml", result.clone());
        assert!(result.contains("class Animal"));
        assert!(result.contains("Animal <|-- Horse"));
        assert!(result.contains("Animal <|-- Snake"));
    }

    #[test]
    fn should_only_have_one_file() {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let ctags_dir = root_dir.join("_fixtures").join("ctags").join("source");

        let path = format!("{}", ctags_dir.display());

        let suffixes = vec!["store".to_string()];

        let option = ParseOption::default();
        let vec = by_dir(
            path,
            FileFilter::new(vec![], suffixes, "".to_string()),
            &option,
        );

        assert_eq!(3, vec.len());
        let result = PlantUmlRender::render(&vec, &option);

        let _ = fs::write("demo.puml", result.clone());
        assert!(!result.contains("class Animal"));
        assert!(result.contains("class classinfo_st"));
    }

    #[test]
    fn should_support_for_rust_property() {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let ctags_dir = root_dir.join("src").join("ctags");

        let path = format!("{}", ctags_dir.display());

        let option = ParseOption::default();
        let vec = by_dir(
            path,
            FileFilter::new(vec![], vec![], "".to_string()),
            &option,
        );

        assert_eq!(3, vec.len());
    }

    #[test]
    fn should_filter_by_grep() {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let ctags_dir = root_dir.join("_fixtures").join("ctags").join("source");

        let path = format!("{}", ctags_dir.display());

        let option = ParseOption::default();
        let filter = FileFilter::new(vec![], vec![], "store.go".to_string());
        let vec = by_dir(path, filter, &option);

        assert_eq!(3, vec.len());
    }

    #[test]
    fn should_render_mermaid() {
        let path = format!("{}", ctags_fixtures_dir().display());
        let option = ParseOption::default();
        let vec = by_dir(path, FileFilter::default(), &option);

        assert_eq!(3, vec.len());
        let result = MermaidRender::render(&vec, &option);

        let _ = fs::write("demo.puml", result.clone());
        assert!(result.contains("class Animal"));
        assert!(result.contains("Animal <|-- Horse"));
        assert!(result.contains("Animal <|-- Snake"));
    }
}
