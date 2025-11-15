import React, { useState } from 'react';

function App() {
    const [recording, setRecording] = useState(false);
    const [audioBlob, setAudioBlob] = useState(null);
    const [transcribed, setTranscribed] = useState('');
    const [sessionId, setSessionId] = useState('');
    const mediaRecorderRef = React.useRef(null);
    const sessionIdRef = React.useRef('');
    const audioChunksRef = React.useRef([]);

    const startRecording = async () => {
      console.log('Starting recording...');
      setTranscribed('');
      setRecording(true);
      audioChunksRef.current = [];
      const newSessionId = Math.random().toString(36).substring(7);
      setSessionId(newSessionId);
      sessionIdRef.current = newSessionId;
      const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
      const options = { mimeType: 'audio/webm;codecs=opus' };
      if (!window.MediaRecorder.isTypeSupported(options.mimeType)) {
        console.log('WebM not supported, using default');
        options.mimeType = '';
      }
      const mediaRecorder = new window.MediaRecorder(stream, options);
      mediaRecorderRef.current = mediaRecorder;
      mediaRecorder.ondataavailable = (e) => {
        console.log('Data available:', e.data.size);
        if (e.data.size > 0) {
          audioChunksRef.current.push(e.data);
          // Send chunk for streaming transcription
          const formData = new FormData();
          formData.append('session_id', sessionIdRef.current);
          formData.append('audio', e.data, 'chunk.webm');
          fetch('/api/transcribe-stream', {
            method: 'POST',
            body: formData,
          }).then(res => {
            if (!res.ok) throw new Error('Network response was not ok');
            return res.json();
          }).then(data => {
            setTranscribed(data.text || '');
          }).catch(err => {
            console.error('Stream error:', err);
            // Don't crash the component
          });
        }
      };
      mediaRecorder.onstop = () => {
        console.log('Recording stopped, creating blob...');
        const blob = new Blob(audioChunksRef.current, { type: 'audio/webm' });
        console.log('Blob created:', blob.size);
        setAudioBlob(blob);
        stream.getTracks().forEach(track => track.stop());
      };
      mediaRecorder.onerror = (e) => {
        console.error('MediaRecorder error:', e);
      };
      mediaRecorder.onstart = () => {
        console.log('MediaRecorder started');
      };
              mediaRecorder.start(1000); // Send chunks every 1 second
    };

    const stopRecording = () => {
      console.log('Stopping recording...');
      if (mediaRecorderRef.current) {
        console.log('MediaRecorder state:', mediaRecorderRef.current.state);
        if (mediaRecorderRef.current.state === 'recording') {
          mediaRecorderRef.current.stop();
        } else {
          console.log('MediaRecorder not recording');
        }
      } else {
        console.log('No MediaRecorder ref');
      }
      setRecording(false);
    };

    const sendAudio = async () => {
      if (!audioBlob) return;
      setTranscribed('Transcribing...');
      const formData = new FormData();
      formData.append('audio', audioBlob, 'audio.wav');
      const res = await fetch('/api/transcribe', {
        method: 'POST',
        body: formData,
      });
      if (res.ok) {
        const data = await res.json();
        setTranscribed(data.text || '(no transcription)');
      } else {
        setTranscribed('Error transcribing audio');
      }
    };
  const [message, setMessage] = useState('');
  const [response, setResponse] = useState('');
  const [loading, setLoading] = useState(false);
  const [model, setModel] = useState('llama2:13b');
  const [history, setHistory] = useState([]);

  const fetchHistory = async () => {
    const res = await fetch('/api/history');
    if (res.ok) {
      const data = await res.json();
      setHistory(data.history || []);
    }
  };

  const sendMessage = async () => {
    setLoading(true);
    try {
      const res = await fetch('/api/chat', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ message, model }),
      });
      if (!res.ok) {
        setResponse(`Error: ${res.status} ${res.statusText}`);
      } else {
        const data = await res.json();
        setResponse(data.response || '(no response)');
        fetchHistory();
      }
    } catch (err) {
      setResponse(`Error: ${err.message}`);
    }
    setLoading(false);
  };

  // Load history on mount
  React.useEffect(() => {
    fetchHistory();
  }, []);

  return (
    <div style={{ padding: 32 }}>
      <h1>AI Companion (Text Chat)</h1>
      <div style={{ marginBottom: 16 }}>
              <div style={{ marginBottom: 16 }}>
                <strong>Voice Input:</strong>
                <div>
                  {!recording ? (
                    <button onClick={startRecording}>Record</button>
                  ) : (
                    <button onClick={stopRecording}>Stop</button>
                  )}
                  {audioBlob && (
                    <button onClick={sendAudio} style={{ marginLeft: 8 }}>Send Audio</button>
                  )}
                </div>
                {transcribed && (
                  <div style={{ marginTop: 8 }}><b>Transcribed:</b> {transcribed}</div>
                )}
              </div>
        <label>
          Model:
          <input
            type="text"
            value={model}
            onChange={e => setModel(e.target.value)}
            style={{ marginLeft: 8 }}
          />
        </label>
      </div>
      <textarea
        rows={4}
        value={message}
        onChange={e => setMessage(e.target.value)}
        placeholder="Type your message..."
        style={{ width: '100%', marginBottom: 16 }}
      />
      <br />
      <button onClick={sendMessage} disabled={loading || !message}>
        {loading ? 'Sending...' : 'Send'}
      </button>
      <div style={{ marginTop: 32 }}>
        <strong>Response:</strong>
        <div>{response}</div>
      </div>
      <div style={{ marginTop: 32 }}>
        <strong>Chat History:</strong>
        <div style={{ background: '#f9f9f9', padding: 16, borderRadius: 8 }}>
          {history.length === 0 ? <em>No history yet.</em> : history.map((entry, i) => (
            <div key={i} style={{ marginBottom: 8 }}>
              <b>{entry.role}:</b> {entry.content}
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}

export default App;
