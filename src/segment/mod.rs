pub mod camelcase_tok;


#[cfg(test)]
mod tests {
    use tokenizers::{OffsetReferential, OffsetType, PreTokenizedString, PreTokenizer};
    use tokenizers::tokenizer::{Tokenizer};
    use crate::segment::camelcase_tok::CamelCaseTok;

    #[test]
    fn should_segmentation() {

        let pretok = CamelCaseTok::default();

        let mut pretokenized = PreTokenizedString::from("HierarchyId");
        pretok.pre_tokenize(&mut pretokenized).unwrap();

        let vec = pretokenized
            .get_splits(OffsetReferential::Original, OffsetType::Byte)
            .into_iter()
            .map(|(s, _o, _)| (s))
            .collect::<Vec<&str>>();

        println!("{:?}", vec);
    }
}