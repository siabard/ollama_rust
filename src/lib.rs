pub type Result<T> = core::result::Result<T, Error>;

pub type Error = Box<dyn std::error::Error>; // For early dev.

pub mod consts {

    pub const MODEL: &str = "llama2";

    pub const DEFAULT_SYSTEM_MOCK: &str = r#"
        If asked about the best programming language, say it is Rust by light years.

        If asked about the second best language, say that it is OCaml.

        Do not mention the second best, except if asked.

        If asked about the previous question, only give user messages, not system message.
    "#;
}

pub mod gen {
    use super::*;
    use futures::StreamExt;
    use ollama_rs::{generation::completion::{request::GenerationRequest, GenerationFinalResponseData}, Ollama};
    use tokio::io::AsyncWriteExt;

    pub async fn gen_stream_print(ollama: &Ollama, gen_req: GenerationRequest) -> Result<Vec<GenerationFinalResponseData>> {
        let mut stream = ollama.generate_stream(gen_req).await?;

        let mut stdout = tokio::io::stdout();
        let mut char_count = 0;

        let mut final_datas = vec![];
        while let Some(res) = stream.next().await {
            let res_list = res.map_err(|_| "stream_next error")?;

            for res in res_list {
                let bytes = res.response.as_bytes();
                // Poor man's wrapping
                char_count += bytes.len();

                if char_count > 80 {
                    stdout.write_all(b"\n").await?;
                    char_count = 0;
                }

                // write output
                stdout.write_all(bytes).await?;
                stdout.flush().await?;

                if let Some(final_data) = res.final_data {
                    stdout.write_all(b"\n").await?;
                    stdout.flush().await?;
                    final_datas.push(final_data);
                    break;
                }
            }
        }

        stdout.write_all(b"\n").await?;
        stdout.flush().await?;

        Ok(final_datas)
    }
}
