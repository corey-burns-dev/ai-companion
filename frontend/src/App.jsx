import { useState } from 'react';

function App() {
  const [message, setMessage] = useState('');
  const [response, setResponse] = useState('');
  const [loading, setLoading] = useState(false);
  const [model, setModel] = useState('llama2:13b');

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
      }
    } catch (err) {
      setResponse(`Error: ${err.message}`);
    }
    setLoading(false);
  };

  return (
    <div style={{ padding: 32 }}>
      <h1>AI Companion (Text Chat)</h1>
      <div style={{ marginBottom: 16 }}>
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
    </div>
  );
}

export default App;
