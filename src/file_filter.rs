use std::path::PathBuf;
use std::ffi::OsStr;

pub fn filter_by_packages(path: PathBuf, packages: Vec<String>) -> bool {
    return false;
}

pub fn filter_by_suffix(path: PathBuf, suffixes: Vec<String>) -> bool {
    if let None = path.file_name() {
        return false;
    }
    let file_name = path.file_name().unwrap().to_str().unwrap();

    for suffix in suffixes.iter() {
        if file_name.contains(suffix) {
            return true;
        }
    }

    return false;
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::file_filter::filter_by_suffix;

    #[test]
    fn should_filter_by_file_name_suffix() {
        let buf = PathBuf::new()
            .join("model")
            .join("CustomModel.java");

        let suffixes = vec!["Model".to_string()];

        assert!(filter_by_suffix(buf, suffixes));
    }
}