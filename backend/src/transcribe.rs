use axum::{Json, extract::Multipart};
use serde::Serialize;
use std::process::Command;
use std::fs::File;
use std::io::Write;
use uuid::Uuid;

#[derive(Serialize)]
pub struct TranscribeResponse {
    pub text: String,
}

pub async fn transcribe_handler(mut multipart: Multipart) -> Json<TranscribeResponse> {
    // Save uploaded audio to temp file
    let mut audio_path = None;
    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.name() == Some("audio") {
            let data = field.bytes().await.unwrap();
            let filename = format!("/tmp/{}.wav", Uuid::new_v4());
            let mut file = File::create(&filename).unwrap();
            file.write_all(&data).unwrap();
            audio_path = Some(filename);
        }
    }
    let audio_path = match audio_path {
        Some(p) => p,
        None => return Json(TranscribeResponse { text: "No audio uploaded".to_string() }),
    };
    // Run whisper.cpp (assumes 'main' binary in PATH)
    let output = Command::new("whisper.cpp")
        .arg("--model")
        .arg("models/ggml-base.en.bin")
        .arg("--file")
        .arg(&audio_path)
        .arg("--output-txt")
        .output();
    let text = match output {
        Ok(out) => String::from_utf8_lossy(&out.stdout).to_string(),
        Err(_) => "Failed to run whisper.cpp".to_string(),
    };
    Json(TranscribeResponse { text })
}
