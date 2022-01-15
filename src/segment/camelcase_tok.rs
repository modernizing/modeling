use tokenizers::{PreTokenizedString, PreTokenizer, SplitDelimiterBehavior};

#[derive(Clone, Debug)]
pub struct CamelCaseTok;

impl Default for CamelCaseTok {
    fn default() -> Self {
        Self
    }
}

impl PreTokenizer for CamelCaseTok {
    fn pre_tokenize(&self, pretokenized: &mut PreTokenizedString) -> tokenizers::Result<()> {
        pretokenized.split(|_, normalized| {
            normalized.split(char::is_uppercase, SplitDelimiterBehavior::Contiguous)
        })
    }
}
