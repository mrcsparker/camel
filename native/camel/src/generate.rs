pub struct CamelGenerate {
    /// Sets the number of threads to use
    /// Default is 4
    pub num_threads: i32,

    /// Number of tokens to predict
    /// Default is 512
    pub num_predict: i64,

    /// How many tokens from the prompt at a time to feed the network. Does not
    /// affect generation.
    /// Default is 8
    pub batch_size: i64,

    /// Size of the 'last N' buffer that is used for the `repeat_penalty`
    /// Default is 64
    pub repeat_last_n: i64,

    /// The penalty for repeating tokens. Higher values make the generation less
    /// likely to get into a loop, but may harm results when repetitive outputs
    /// are desired.
    /// Default is 1.30
    pub repeat_penalty: f64,

    /// Temperature, higher is more creative, should between 0 to 1
    /// Default is 0.80
    pub temperature: f64,

    /// Top-K: The top K words by score are kept during sampling.
    /// Default is 40
    pub top_k: i64,

    /// Top-p: The cumulative probability after which no more words are kept
    /// for sampling.
    /// Default is 0.95
    pub top_p: f64,

    /// Specifies the seed to use during sampling. Note that, depending on
    /// hardware, the same seed may lead to different results on two separate
    /// machines.
    /// Default is None
    pub seed: Option<i64>,

    /// Use 16-bit floats for model memory key and value. Ignored when restoring
    /// from the cache.
    /// Default is false
    pub float16: bool,

    /// Prompt for inference
    pub prompt: String,

    /// A comma separated list of token biases. The list should be in the format
    /// "TID=BIAS,TID=BIAS" where TID is an integer token ID and BIAS is a
    /// floating point number.
    /// For example, "1=-1.0,2=-1.0" sets the bias for token IDs 1
    /// (start of document) and 2 (end of document) to -1.0 which effectively
    /// disables the model from generating responses containing those token IDs.
    /// Default is None
    pub token_bias: Option<Vec<TokenBias>>,

    /// Prevent the end of stream (EOS/EOD) token from being generated. This will allow the
    /// model to generate text until it runs out of context space. Note: The --token-bias
    /// option will override this if specified.
    /// Default is false
    pub ignore_eos: bool,

    /// Feed prompt before inference, will hide feeded tokens in inference result
    /// Default is false
    pub feed_prompt: bool,

    /// Only feed prompt, will not execute inference
    /// When feed_prompt_only is true, feed_prompt will always be true
    /// Default is false
    pub feed_prompt_only: bool,

    /// Load session path
    /// Default is None
    pub load_session: Option<String>,

    /// Persist session path
    /// Default is None
    pub save_session: Option<String>,
}

impl Default for CamelGenerate {
    fn default() -> Self {
        Self {
            num_threads: num_cpus::get_physical() as i32,
            num_predict: 512,
            batch_size: 8,
            repeat_last_n: 64,
            repeat_penalty: 1.30,
            temperature: 0.80,
            top_k: 40,
            top_p: 0.95,
            seed: None,
            float16: false,
            prompt: "".to_string(),
            token_bias: None,
            ignore_eos: false,
            feed_prompt: false,
            feed_prompt_only: false,
            load_session: None,
            save_session: None,
        }
    }
}
