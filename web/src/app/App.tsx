import { useState } from "react"
import { greetFromRust } from "../lib/wasm/calculator";

function App() {
  const [message, setMessage] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function handleGreet() {
    setLoading(true);
    setError(null);

    try {
      const result = await greetFromRust("World");
      setMessage(result);
    } catch (err) {
      setError(
        err instanceof Error ? err.message : 'WASM initialization failed'
      );
    } finally {
      setLoading(false);
    }
  }

  return (
    <main className="min-h-screen flex flex-col gap-6 items-center justify-center bg-slate-800">
      <h1 className="text-3xl font-bold text-slate-300">
        Calculator Remastered
      </h1>

      <button
        onClick={handleGreet}
        disabled={loading}
        className="rounded-lg bg-sky-900 px-5 py-3 text-sky-200 disabled:opacity-50"
      >
        {loading ? 'Loading WASM...' : 'Call Rust'}
      </button>

      {message.trim() === '' ?
        <p className="text-slate-200">Messages will appear here...</p> :
        <p className="text-slate-200">{message}</p>
      }

      {error && (<p className="text-red-500">{error}</p>)}
    </main>
  )
}

export default App
