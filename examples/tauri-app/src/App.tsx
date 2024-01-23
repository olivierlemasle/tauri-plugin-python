import { useState } from "react";
import { importModule } from "tauri-plugin-python-api";
import "./App.css";

function App() {
  const [modulePath, setModulePath] = useState("");
  const [responses, setResponses] = useState<string[]>([]);

  async function _importModule() {
    try {
      await importModule(modulePath);
      setResponses([...responses, `[${new Date().toLocaleTimeString()}] OK`]);
    } catch (err: any) {
      setResponses([
        ...responses,
        `[${new Date().toLocaleTimeString()}] ` + JSON.stringify(err),
      ]);
    } finally {
      setModulePath("");
    }
  }

  return (
    <div className="container">
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          _importModule();
        }}
      >
        <input
          id="module-path-input"
          onChange={(e) => setModulePath(e.currentTarget.value)}
          placeholder="Enter a path..."
        />
        <button type="submit">Import module</button>
      </form>

      <div>
        {responses.map((s, i) => (
          <p key={i}>{s}</p>
        ))}
      </div>
    </div>
  );
}

export default App;
