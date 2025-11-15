# Local AI Companion Project

## Vision

A fully local, privacy-first AI companion with:

- Natural conversation (local LLM)
- Voice input/output
- Visual avatar
- Persistent memory/personality
- Modular, step-by-step development

---

## Technical Architecture (Text Diagram)

**Core Components:**

- **Local LLM Backend:** Ollama or LM Studio (chat, personality, memory)
- **Voice Input:** Whisper.cpp (speech-to-text)
- **Voice Output:** Piper (text-to-speech)
- **Avatar UI:** Web (Three.js/React) or Desktop (Godot/Unity/Electron)
- **Memory Store:** Local database (SQLite, JSON, LiteDB)
- **Controller/Glue:** Python, Node.js, or Rust

**Data Flow:**
User speaks → Whisper.cpp → LLM (Ollama) → Response → Piper (TTS) → Avatar animates/talks → Memory updated

---

## MVP Feature Breakdown

**Step 1:** Text chat with local LLM (Ollama)
**Step 2:** Add voice input (Whisper.cpp) and output (Piper)
**Step 3:** Integrate basic avatar (web or desktop)
**Step 4:** Add persistent memory (local chat history)
**Step 5:** Expand avatar animation, personality, context

---

## Development Plan

We will build and test each step before moving on:

1. **Text chat prototype:** CLI or web app, connect to local LLM
2. **Voice integration:** Add speech-to-text and text-to-speech
3. **Avatar UI:** Display and animate a character
4. **Memory:** Store and retrieve conversation history
5. **Advanced features:** Personality, context, roleplay modes

---

## Next Step

Start with Step 1: Text chat with local LLM. Ensure it works before adding voice.

---

## Progress Log

- [ ] Step 1: Text chat prototype
- [ ] Step 2: Voice input/output
- [ ] Step 3: Avatar UI
- [ ] Step 4: Memory
- [ ] Step 5: Advanced features

---

*Update this file as you build each step.*
