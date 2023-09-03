use llm::models::Llama;
use llm_base::{Model, Prompt};
use std::io::Write;

pub mod camel;

#[rustler::nif]
fn get_accelerator() -> String {
    match llm_base::ggml::accelerator::get_accelerator() {
        llm_base::ggml::accelerator::Accelerator::CuBLAS => "cuda".to_owned(),
        llm_base::ggml::accelerator::Accelerator::CLBlast => "opencl".to_owned(),
        llm_base::ggml::accelerator::Accelerator::Metal => "metal".to_owned(),
        _ => "cpu".to_owned(),
    }
}

#[rustler::nif]
fn simple_chat(model_path: String, prompt: String) {
    let llama = llm::load::<Llama>(
        std::path::Path::new(model_path.as_str()),
        llm::TokenizerSource::Embedded,
        Default::default(),
        llm::load_progress_callback_stdout,
    )
    .unwrap_or_else(|err| panic!("Failed to load model: {err}"));

    let mut session = llama.start_session(Default::default());
    let res = session.infer::<std::convert::Infallible>(
        &llama,
        &mut rand::thread_rng(),
        &llm::InferenceRequest {
            prompt: Prompt::Text(&prompt),
            parameters: &llm::InferenceParameters::default(),
            play_back_previous_tokens: false,
            maximum_token_count: None,
        },
        &mut Default::default(),
        |r| match r {
            llm::InferenceResponse::PromptToken(t) | llm::InferenceResponse::InferredToken(t) => {
                print!("{t}");
                std::io::stdout().flush().unwrap();

                Ok(llm::InferenceFeedback::Continue)
            }
            _ => Ok(llm::InferenceFeedback::Continue),
        },
    );

    match res {
        Ok(result) => println!("\n\nInference stats:\n{result}"),
        Err(err) => println!("\n{err}"),
    }
}

rustler::init!(
    "Elixir.Ellem",
    [
        get_accelerator,
        simple_chat,
        camel::camel,
        camel::model_path,
        camel::generate
    ],
    load = load
);

fn load(env: rustler::Env, _: rustler::Term) -> bool {
    rustler::resource!(camel::Camel, env);
    true
}
