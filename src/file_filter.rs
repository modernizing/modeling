use std::path::PathBuf;

pub fn no_filter(_path: PathBuf, _packages: Vec<String>) -> bool {
    return true;
}

pub fn filter_by_packages(path: PathBuf, packages: Vec<String>) -> bool {
    if packages.len() == 0 {
        return true;
    }

    for child in path.iter() {
        return match child.to_str() {
            None => { false }
            Some(sub) => {
                packages.contains(&sub.to_string())
            }
        }
    }

    return false;
}

pub fn filter_by_suffix(path: PathBuf, suffixes: Vec<String>) -> bool {
    if suffixes.len() == 0 {
        return true;
    }

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
    use crate::file_filter::{filter_by_suffix, filter_by_packages};

    #[test]
    fn should_filter_by_file_name_suffix() {
        let buf = PathBuf::new().join("model").join("CustomModel.java");
        let suffixes = vec!["Model".to_string()];

        assert!(filter_by_suffix(buf, suffixes));
    }

    #[test]
    fn should_return_false_when_not_correct_name() {
        let buf = PathBuf::new().join("controller").join("CustomController.java");
        let suffixes = vec!["Model".to_string()];

        assert_eq!(false, filter_by_suffix(buf, suffixes));
    }

    #[test]
    fn should_no_filter_for_empty_suffix() {
        let buf = PathBuf::new().join("controller").join("CustomController.java");
        let suffixes: Vec<String> = vec![];

        assert_eq!(true, filter_by_suffix(buf, suffixes));
    }

    #[test]
    fn should_filter_by_package() {
        let buf = PathBuf::new()
            .join("model")
            .join("CustomModel.java");

        let suffixes = vec!["model".to_string()];

        assert!(filter_by_packages(buf, suffixes));
    }

    #[test]
    fn should_return_no_when_no_in_dir() {
        let buf = PathBuf::new()
            .join("model")
            .join("CustomModel.java");

        let suffixes = vec!["controller".to_string()];

        assert_eq!(false, filter_by_packages(buf, suffixes));
    }

    #[test]
    fn should_no_filter_for_empty_package() {
        let buf = PathBuf::new()
            .join("model")
            .join("CustomModel.java");

        let suffixes: Vec<String> = vec![];

        assert!(filter_by_packages(buf, suffixes));
    }
}