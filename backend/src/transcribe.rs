use axum::{Json, extract::Multipart};
use serde::Serialize;
use std::process::Command;
use std::fs::File;
use std::io::Write;

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
            let filename = "uploaded_audio.wav".to_string(); // Save to backend directory for testing
            let mut file = File::create(&filename).unwrap();
            file.write_all(&data).unwrap();
            audio_path = Some(filename);
        }
    }
    let audio_path = match audio_path {
        Some(p) => p,
        std::prelude::v1::None => return Json(TranscribeResponse { text: "No audio uploaded".to_string() }),
    };
    // Convert audio to WAV using FFmpeg (in case it's not proper WAV)
    let converted_path = format!("{}_converted.wav", audio_path);
    let ffmpeg_output = Command::new("ffmpeg")
        .arg("-i")
        .arg(&audio_path)
        .arg("-acodec")
        .arg("pcm_s16le")
        .arg("-ar")
        .arg("16000")
        .arg("-ac")
        .arg("1")
        .arg("-y")
        .arg(&converted_path)
        .output();
    if ffmpeg_output.is_err() {
        return Json(TranscribeResponse { text: "Failed to convert audio".to_string() });
    }
    // Run whisper-cli with correct model and converted file path
    let output = Command::new("whisper-cli")
        .arg("-m")
        .arg("/home/cburns/ideas/ai-companion/backend/models/ggml-base.en.bin")
        .arg("-f")
        .arg(&converted_path)
        .output();
    let text = match output {
        Ok(out) => String::from_utf8_lossy(&out.stdout).to_string(),
        Err(_) => "Failed to run whisper-cli".to_string(),
    };
    Json(TranscribeResponse { text })
}
