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

    pub fn embeddings(&self) -> Vec<f64> {
        let model = self.model.as_ref();

        let default_inference_session_config = InferenceSessionConfig::default();
        let config = InferenceSessionConfig {
            memory_k_type: default_inference_session_config.memory_k_type,
            memory_v_type: default_inference_session_config.memory_v_type,
            n_batch: default_inference_session_config.n_batch,
            n_threads: num_cpus::get_physical(),
        };
        let mut session = self.model.start_session(config);

        if let Err(InferenceError::ContextFull) = session.feed_prompt::<Infallible, &str>(
            model,
            "What is your name?",
            &mut Default::default(),
            |_| Ok(InferenceFeedback::Continue),
        ) {
            panic!("Context window full");
        }

        let end_token = self.tokenize("\n");

        let mut output_request = OutputRequest {
            all_logits: None,
            embeddings: Some(Vec::new()),
        };

        model.evaluate(&mut session, &end_token, &mut output_request);

        let output: Option<Vec<f64>> = output_request
            .embeddings
            .map(|embd| embd.into_iter().map(|data| data.into()).collect());

        output.unwrap_or_default()
    }
}
