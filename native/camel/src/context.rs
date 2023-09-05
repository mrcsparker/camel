use llm::models::Llama;
use llm::Model;

use crate::load::CamelLoad;

pub struct CamelContext {
    pub model: Llama,
}

impl CamelContext {
    pub fn new(params: &CamelLoad) -> CamelContext {
        let model = params.load();
        CamelContext { model }
    }

    pub fn tokenize(&self, text: &str) -> Vec<u32> {
        let tokens = self.model.tokenizer().tokenize(text, false).unwrap();
        tokens
            .iter()
            .map(|(_, token_id)| *token_id)
            .collect::<Vec<_>>()
    }
}
