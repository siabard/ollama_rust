use futures::StreamExt;
use ollama_service::{
    consts::{DEFAULT_SYSTEM_MOCK, MODEL},
    gen::gen_stream_print,
    Result,
};

use ollama_rs::{generation::completion::{request::GenerationRequest, GenerationContext}, Ollama};
use simple_fs::{ensure_dir, ensure_file_dir, save_json};
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> Result<()> {
    let ollama = Ollama::default();

    let model = MODEL.to_string();
    let prompts = &[
        "Why the sky is red? (be concise)",
        "What was my first question?",
    ];

    let mut last_ctx: Option<GenerationContext> = None;

    for prompt in prompts {
        println!("->> {prompt}");
        let mut gen_req = GenerationRequest::new(model.clone(), prompt.to_string());

        if let Some(last_ctx) = last_ctx.take() {
            gen_req = gen_req.context(last_ctx);
        }

        let mut final_data_list = gen_stream_print(&ollama, gen_req).await?;

        if let Some(final_data) = final_data_list.pop() {
            last_ctx = Some(final_data.context);

            // save context for debug
            let ctx_file_path = ".c02-data/ctx.json";
            ensure_file_dir(ctx_file_path)?;
            save_json(ctx_file_path,&last_ctx)?;

        }
    }
    Ok(())
}
