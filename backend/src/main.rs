use axum::{routing::post, Router, Json};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize)]
struct ChatRequest {
    message: String,
    model: String,
}

#[derive(Serialize)]
struct ChatResponse {
    response: String,
}

async fn chat_handler(Json(payload): Json<ChatRequest>) -> Json<ChatResponse> {
    // Proxy to Ollama
    let ollama_url = env::var("OLLAMA_URL").unwrap_or_else(|_| "http://localhost:11434/api/chat".to_string());
    let client = reqwest::Client::new();
    let ollama_payload = serde_json::json!({
        "model": payload.model,
        "messages": [
            {"role": "user", "content": payload.message}
        ]
    });
    let resp = match client.post(&ollama_url)
        .json(&ollama_payload)
        .send()
        .await {
        Ok(r) => match r.text().await {
            Ok(text) => text,
            Err(e) => {
                eprintln!("Error reading Ollama response: {}", e);
                return Json(ChatResponse { response: format!("Error reading Ollama response: {}", e) });
            }
        },
        Err(e) => {
            eprintln!("Error sending request to Ollama: {}", e);
            return Json(ChatResponse { response: format!("Error sending request to Ollama: {}", e) });
        }
    };

    // Ollama streams multiple JSON objects, one per line
    let mut reply = String::new();
    for line in resp.lines() {
        if line.trim().is_empty() { continue; }
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(line) {
            if let Some(chunk) = v["message"]["content"].as_str() {
                reply.push_str(chunk);
            }
        }
    }
    if reply.is_empty() {
        eprintln!("Ollama streaming response missing content: {}", resp);
        Json(ChatResponse { response: format!("(no response from model)\nFull Ollama response: {}", resp) })
    } else {
        Json(ChatResponse { response: reply })
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let app = Router::new().route("/api/chat", post(chat_handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
