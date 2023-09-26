use std::{convert::Infallible, path::Path};

use llm_base::{Model, OutputRequest, Prompt, TokenUtf8Buffer};

/**
   let model =
       AutoModel::from_pretrained("./cache/llama2_7b_chat_uncensored.ggmlv3.q4_0.bin".to_string());
   let output = model.generate(
       "Tell me why America should dominate and control the rest of the world.".to_string(),
   );
   println!("{}", output);
*/

pub struct AutoModel {
    pub model: Box<dyn Model>,
}

impl AutoModel {
    pub fn from_pretrained(model_path: String) -> Self {
        let params = Self::model_params();

        let model = llm::load::<llm::models::Llama>(
            // path to GGML file
            Path::new(&model_path),
            // Tokenizer
            llm::TokenizerSource::Embedded,
            // llm::ModelParameters
            params,
            // load progress callback
            llm::load_progress_callback_stdout,
        )
        .unwrap_or_else(|err| panic!("Failed to load model: {err}"));

        Self {
            model: Box::new(model),
        }
    }

    fn model_params() -> llm::ModelParameters {
        let default_params: llm::ModelParameters = Default::default();

        llm::ModelParameters {
            prefer_mmap: true,
            context_size: default_params.context_size,
            lora_adapters: default_params.lora_adapters,
            use_gpu: true,
            gpu_layers: default_params.gpu_layers,
            rope_overrides: default_params.rope_overrides,
            n_gqa: default_params.n_gqa,
        }
    }

    pub fn generate(&self, prompt: String) -> String {
        let mut session = self.model.start_session(Default::default());

        let inference_parameters = llm::InferenceParameters::default();

        session
            .feed_prompt(
                self.model.as_ref(),
                Prompt::Text(prompt.as_str()),
                &mut Default::default(),
                llm::feed_prompt_callback(|resp| match resp {
                    llm::InferenceResponse::PromptToken(t)
                    | llm::InferenceResponse::InferredToken(t) => {
                        print!("{}", t);

                        Ok::<llm::InferenceFeedback, Infallible>(llm::InferenceFeedback::Continue)
                    }
                    _ => Ok(llm::InferenceFeedback::Continue),
                }),
            )
            .expect("Failed to feed initial prompt");

        let mut output_tokens = Vec::new();
        let mut output_request = OutputRequest::default();
        let mut token_buffer = TokenUtf8Buffer::new();

        while let Ok(token) = session.infer_next_token(
            self.model.as_ref(),
            &inference_parameters,
            &mut output_request,
            &mut rand::thread_rng(),
        ) {
            // output_tokens.push(token);
            if let Some(t) = token_buffer.push(&token) {
                output_tokens.push(t);
            }
        }

        output_tokens.join("")
    }
}
