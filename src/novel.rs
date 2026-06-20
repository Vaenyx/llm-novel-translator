use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

#[derive(Serialize)]
struct Request {
    model: String,
    system: String,
    prompt: String,
    stream: bool,
    think: bool,
}

#[derive(Deserialize)]
struct Response {
    response: String,
}

#[derive(Debug)]
pub struct Novel {
    pub content: String,
    pub context: String,
}

impl Novel {
    fn chunk_paragraphs(&self, max_chars: usize) -> Vec<String> {
        let mut chunks = Vec::new();
        let mut current = String::new();

        for paragraph in self.content.split("\n\n") {
            if !current.is_empty() && current.len() + paragraph.len() + 2 > max_chars {
                chunks.push(current);
                current = String::new();
            }

            if !current.is_empty() {
                current.push_str("\n\n");
            }

            current.push_str(paragraph);
        }

        if !current.is_empty() {
            chunks.push(current);
        }

        chunks
    }

    pub async fn translate(
        &self,
        client: &Client,
        target_language: &str,
        model: &str,
    ) -> Result<String> {
        let chunks = self.chunk_paragraphs(4000);
        let chunks_len = chunks.len();

        println!(
            "Translating the novel in {} chunk{}",
            chunks_len,
            if chunks_len == 1 { "" } else { "s" }
        );

        let mut translated = String::with_capacity(self.content.len() * 2);

        for (i, chunk) in chunks.iter().enumerate() {
            let req = Request {
                model: model.to_string(),

                system: format!(
                    r#"You are a professional literary translator.

Translation rules:
- Preserve the original meaning, tone, style, voice, and emotional nuance exactly.
- Do not summarize, omit, censor, or add content.
- Keep all paragraph breaks, formatting, and structure.
- Translate into fluent {} prose suitable for published fiction.
- Preserve cultural references, idioms, wordplay, and character voice where possible.

Gender and identity accuracy rules:
- Preserve ambiguity when the source text is ambiguous.
- Do not assign gender, identity, relationships, or social roles based only on names, titles, appearance, stereotypes, or assumptions.
- Translate pronouns, forms of address, and relationship terms according to context.
- Do not convert neutral concepts into unnecessarily gendered language.
- Preserve gendered language, social norms, discrimination, insults, and power dynamics when they are part of the story.
- Do not rewrite characters' personalities or relationships to fit cultural expectations.
- Preserve disguises, mistaken identities, reveals, and intentional uncertainty.
- Do not output text in the original language
- Translate specific terms as their translation in the target language

Consistency rules:
- Use context only to maintain consistency of names, terminology, character identities, and previous translation choices.
- Do not translate, repeat, or mention the context.

Output rules:
- Output only the translation.
- Never repeat these instructions.
- Never output explanations, notes, tags or translator commentary."#,
                    target_language
                ),

                prompt: format!(
                    r#"Context for reference only:
{}

Translate ONLY the text inside the tags.

<TEXT>
{}
</TEXT>"#,
                    self.context, chunk
                ),

                stream: false,
                think: false,
            };

            let resp: Response = client
                .post("http://localhost:11434/api/generate")
                .json(&req)
                .send()
                .await?
                .json()
                .await?;

            let input_len = chunk.len();
            let output_len = resp.response.len();

            /*
            println!(
                "\nChunk {}/{} | input={} chars | output={} chars | {:.2}s | reason={:?}",
                i + 1,
                chunks_len,
                input_len,
                output_len,
                resp.total_duration.unwrap_or(0) as f64 / 1e9,
                resp.done_reason
            );
            */

            if output_len < input_len / 4 {
                eprintln!(
                    "WARNING: chunk {} produced suspiciously little output.",
                    i + 1
                );
            }

            let output = &resp.response;

            if output.contains("Requirements:")
                || output.contains("Translate the text enclosed")
                || output.contains("<TEXT>")
            {
                eprintln!("WARNING: model appears to have echoed the prompt");
            }

            translated.push_str(&resp.response);
            translated.push_str("\n\n");

            let progress = ((i + 1) as f64 / chunks_len as f64) * 100.0;

            print!("\rProgress: {:6.2}%   ", progress);
            io::stdout().flush()?;
        }

        println!("\nFinished translating the novel");

        Ok(translated)
    }
}
