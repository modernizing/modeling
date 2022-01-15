pub mod camelcase_tok;

use tokenizers::{OffsetReferential, OffsetType, PreTokenizedString, PreTokenizer};
use crate::segment::camelcase_tok::CamelCaseTok;

pub fn segment_camelcase() -> Vec<&str> {
    let pretok = CamelCaseTok::default();

    let mut pretokenized = PreTokenizedString::from("HierarchyId");
    pretok.pre_tokenize(&mut pretokenized).unwrap();

    pretokenized
        .get_splits(OffsetReferential::Original, OffsetType::Byte)
        .into_iter()
        .map(|(s, _o, _)| (s))
        .collect::<Vec<&str>>()
}


#[cfg(test)]
mod tests {
    use crate::segment::segment_camelcase;

    #[test]
    fn should_segmentation() {
        assert_eq!(vec!["Hierarchy", "Id"], segment_camelcase());
    }
}