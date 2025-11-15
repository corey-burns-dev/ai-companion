use axum::{Json, extract::Multipart};
use serde::Serialize;
use std::process::Command;
use std::fs::OpenOptions;
use std::io::Write;
use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;

#[derive(Serialize)]
pub struct TranscribeStreamResponse {
    pub text: String,
}

lazy_static! {
    static ref AUDIO_BUFFERS: Mutex<HashMap<String, Vec<u8>>> = Mutex::new(HashMap::new());
}

pub async fn transcribe_stream_handler(mut multipart: Multipart) -> Json<TranscribeStreamResponse> {
    let mut session_id = String::new();
    let mut audio_data = Vec::new();

    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.name() == Some("session_id") {
            session_id = field.text().await.unwrap();
        } else if field.name() == Some("audio") {
            audio_data = field.bytes().await.unwrap().to_vec();
        }
    }

    if session_id.is_empty() {
        return Json(TranscribeStreamResponse { text: "No session ID".to_string() });
    }

    // Accumulate audio data
    {
        let mut buffers = AUDIO_BUFFERS.lock().unwrap();
        let buffer = buffers.entry(session_id.clone()).or_insert(Vec::new());
        buffer.extend(&audio_data);
    }

    // Write accumulated data to temp file
    let filename = format!("/tmp/stream_{}.wav", session_id);
    {
        let buffers = AUDIO_BUFFERS.lock().unwrap();
        if let Some(buffer) = buffers.get(&session_id) {
            let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(&filename).unwrap();
            file.write_all(buffer).unwrap();
        }
    }

    // Convert to WAV
    let converted_path = format!("{}_converted.wav", filename);
    let ffmpeg_output = Command::new("ffmpeg")
        .arg("-i")
        .arg(&filename)
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
        return Json(TranscribeStreamResponse { text: "Failed to convert audio".to_string() });
    }

    // Transcribe
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

    Json(TranscribeStreamResponse { text })
}