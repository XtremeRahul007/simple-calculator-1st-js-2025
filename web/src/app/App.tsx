import { useState } from "react";
import {
  greetFromRust,
  calculateExpression,
} from "../lib/wasm/calculator";

function App() {
  const [message, setMessage] = useState("");
  const [text, setText] = useState("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function handleGreet() {
    setLoading(true);
    setError(null);

    try {
      const result = await greetFromRust("😀");
      setMessage(result.greet);
      setText(result.text);
    } catch (err) {
      setError(
        err instanceof Error ? err.message : "WASM initialization failed",
      );
    } finally {
      setLoading(false);
    }
  }

  async function testDivision() {
    const success = await calculateExpression(["16"], "square_root");
    console.log("Success:", success);

    const failure = await calculateExpression(["27", "3"], "nth_root");
    console.log("Failure:", failure);

    if (success.status === "success") {
      console.log("Result:", success.value);
    }

    if (failure.status === "error") {
      console.log("Error code:", failure.error.code);
      console.log("Error message:", failure.error.message);
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
        {loading ? "Loading WASM..." : "Call Rust"}
      </button>

      {message.trim() === "" ? (
        <p className="text-slate-200">Messages will appear here...</p>
      ) : (
        <p className="text-slate-200">{message}</p>
      )}

      {<p className="text-slate-200">{text}</p>}

      {error && <p className="text-red-500">{error}</p>}

      <button
        onClick={testDivision}
        className="rounded-lg bg-sky-900 px-5 py-3 text-sky-200 disabled:opacity-50"
      >
        Test Rust Division
      </button>
    </main>
  );
}

export default App;
