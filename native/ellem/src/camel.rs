use std::{io::Write, path::PathBuf};

use llm_base::{ggml, KnownModel, Prompt};
use rustler::{NifStruct, ResourceArc};

#[derive(Debug, NifStruct)]
#[module = "Camel"]
pub struct Camel {
    model_path: String,
}

impl Camel {
    pub fn new(model_path: String) -> Self {
        Self {
            model_path: model_path.to_string(),
        }
    }

    pub fn model_parameters(&self) -> llm::ModelParameters {
        let camel_model_parameters = CamelModelParameters::default();
        llm::ModelParameters {
            prefer_mmap: camel_model_parameters.prefer_mmap,
            context_size: camel_model_parameters.context_size,
            lora_adapters: camel_model_parameters.lora_adapters.clone(),
            use_gpu: camel_model_parameters.use_gpu,
            gpu_layers: camel_model_parameters.gpu_layers,
            rope_overrides: camel_model_parameters.rope_overrides.clone(),
            n_gqa: camel_model_parameters.n_gqa,
        }
    }

    pub fn llama(&self) -> llm::models::Llama {
        llm::load::<llm::models::Llama>(
            std::path::Path::new(self.model_path.as_str()),
            llm::TokenizerSource::Embedded,
            self.model_parameters(),
            llm::load_progress_callback_stdout,
        )
        .unwrap_or_else(|err| panic!("Failed to load model: {err}"))
    }
}

pub struct CamelModelParameters {
    pub prefer_mmap: bool,
    pub context_size: usize,
    pub lora_adapters: Option<Vec<PathBuf>>,
    pub use_gpu: bool,
    pub gpu_layers: Option<usize>,
    pub rope_overrides: Option<ggml::RoPEOverrides>,
    pub n_gqa: Option<usize>,
}

impl Default for CamelModelParameters {
    fn default() -> Self {
        Self {
            prefer_mmap: true,
            context_size: 2048,
            lora_adapters: None,
            use_gpu: true,
            gpu_layers: None,
            rope_overrides: None,
            n_gqa: None,
        }
    }
}

#[rustler::nif]
pub fn camel(model_path: String) -> ResourceArc<Camel> {
    ResourceArc::new(Camel::new(model_path))
}

#[rustler::nif]
pub fn generate(camel_resource: ResourceArc<Camel>, prompt: String) -> ResourceArc<Camel> {
    let llama = camel_resource.llama();

    let mut session = llama.start_session(Default::default());
    let result = session.infer::<std::convert::Infallible>(
        &llama,
        &mut rand::thread_rng(),
        &llm::InferenceRequest {
            prompt: Prompt::Text(&prompt),
            parameters: &llm::InferenceParameters::default(),
            play_back_previous_tokens: false,
            maximum_token_count: None,
        },
        &mut Default::default(),
        |t| {
            match t {
                llm::InferenceResponse::SnapshotToken(_) => { /*print!("{token}");*/ }
                llm::InferenceResponse::PromptToken(_) => { /*print!("{token}");*/ }
                llm::InferenceResponse::InferredToken(token) => {
                    println!("{token}");
                }
                llm::InferenceResponse::EotToken => {}
            }
            std::io::stdout().flush().unwrap();
            Ok(llm::InferenceFeedback::Continue)
        },
    );

    match result {
        Ok(result) => println!("\n\nInference stats:\n{result}"),
        Err(err) => println!("\n{err}"),
    }

    camel_resource
}
