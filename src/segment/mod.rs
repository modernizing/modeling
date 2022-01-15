pub mod camelcase_tok;

use tokenizers::{OffsetReferential, OffsetType, PreTokenizedString, PreTokenizer};
use crate::segment::camelcase_tok::CamelCaseTok;

pub fn segment(str: &str) -> Vec<String> {
    segment_camelcase(str)
}

pub fn segment_camelcase(str: &str) -> Vec<String> {
    let pretok = CamelCaseTok::default();

    let mut pretokenized = PreTokenizedString::from(str);
    pretok.pre_tokenize(&mut pretokenized).unwrap();

    let vec = pretokenized
        .get_splits(OffsetReferential::Original, OffsetType::Byte)
        .into_iter()
        .map(|(s, _o, _)| (s.to_string()))
        .collect::<Vec<String>>();

    vec
}


#[cfg(test)]
mod tests {
    use crate::segment::{segment};

    #[test]
    fn should_segmentation() {
        assert_eq!(vec!["Hierarchy".to_string(), "Id".to_string()], segment("HierarchyId"));
    }
}