use llm::models::Llama;
use llm::Model;
use llm_base::{InferenceError, InferenceFeedback, InferenceSessionConfig, OutputRequest};
use std::{convert::Infallible, io::Write, path::PathBuf};

use crate::load::CamelLoad;

pub struct CamelContext {
    pub model: Box<dyn Model>,
}

impl CamelContext {
    pub fn new(params: &CamelLoad) -> CamelContext {
        let model = params.load();
        CamelContext { model }
    }

    pub fn tokenize(&self, text: &str) -> Vec<u32> {
        let tokens = self
            .model
            .as_ref()
            .tokenizer()
            .tokenize(text, false)
            .unwrap();
        tokens
            .iter()
            .map(|(_, token_id)| *token_id)
            .collect::<Vec<_>>()
    }

    pub fn embeddings(&self, query: &str) -> Vec<f32> {
        let model = self.model.as_ref();

        let default_inference_session_config = InferenceSessionConfig::default();
        let config = InferenceSessionConfig {
            memory_k_type: default_inference_session_config.memory_k_type,
            memory_v_type: default_inference_session_config.memory_v_type,
            n_batch: default_inference_session_config.n_batch,
            n_threads: num_cpus::get_physical(),
        };
        let mut session = self.model.start_session(config);

        let mut output_request = OutputRequest {
            all_logits: None,
            embeddings: Some(Vec::new()),
        };

        let vocab = model.tokenizer();
        let beginning_of_sentence = true;

        let query_token_ids = vocab
            .tokenize(query, beginning_of_sentence)
            .unwrap()
            .iter()
            .map(|(_, tok)| *tok)
            .collect::<Vec<_>>();
        model.evaluate(&mut session, &query_token_ids, &mut output_request);
        output_request.embeddings.unwrap()
    }
}
