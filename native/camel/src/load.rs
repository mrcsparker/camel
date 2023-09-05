use std::path::Path;

use llm::models::Llama;
use llm::Model;
use llm_base::ModelParameters;

pub struct CamelLoad {
    /// Path of the model
    pub model_path: String,

    /// Sets the size of the context (in tokens). Allows feeding longer prompts.
    /// Note that this affects memory.
    ///
    /// LLaMA models are trained with a context size of 2048 tokens. If you
    /// want to use a larger context size, you will need to retrain the model,
    /// or use a model that was trained with a larger context size.
    ///
    /// Alternate methods to extend the context, including
    /// [context clearing](https://github.com/rustformers/llm/issues/77) are
    /// being investigated, but are not yet implemented. Additionally, these
    /// will likely not perform as well as a model with a larger context size.
    /// Default is 2048
    pub num_ctx_tokens: Option<i64>,

    /// MMapped files are faster, but may not work on all systems.
    /// Default is true
    pub use_mmap: Option<bool>,

    /// Path to the Lora file to apply to the model
    /// Default is None
    pub lora_path: Option<String>,

    /// Use the GPU if available
    pub use_gpu: Option<bool>,
}

impl CamelLoad {
    pub fn load(&self) -> Box<dyn Model> {
        let params = self.params();

        let path = Path::new(&self.model_path);

        let model = llm::load::<llm::models::Llama>(
            // path to GGML file
            path,
            // Tokenizer
            llm::TokenizerSource::Embedded,
            // llm::ModelParameters
            params,
            // load progress callback
            llm::load_progress_callback_stdout,
        )
        .unwrap_or_else(|err| panic!("Failed to load model: {err}"));
        Box::new(model)
    }

    fn params(&self) -> ModelParameters {
        let default_params: ModelParameters = Default::default();

        let params = ModelParameters {
            prefer_mmap: self.use_mmap.unwrap_or(default_params.prefer_mmap),
            context_size: self
                .num_ctx_tokens
                .unwrap_or(default_params.context_size as i64) as usize,
            lora_adapters: self
                .lora_path
                .as_ref()
                .map(|path| vec![path.into()])
                .or(default_params.lora_adapters),
            use_gpu: self.use_gpu.unwrap_or(default_params.use_gpu),
            gpu_layers: default_params.gpu_layers,
            rope_overrides: default_params.rope_overrides,
            n_gqa: default_params.n_gqa,
        };
        params
    }
}
