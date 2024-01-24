import { useState } from "react";
import { importModule, callFunction } from "tauri-plugin-python-api";
import "./App.css";

interface Response {
  timestamp: Date;
  lines: string[];
  isError: boolean;
}

function App() {
  const [modulePath, setModulePath] = useState("");
  const [moduleName, setModuleName] = useState("");
  const [functionName, setFunctionName] = useState("");
  const [responses, setResponses] = useState<Response[]>([]);

  function write(o: any, isError = false) {
    let message = "OK";
    if (o) {
      message = typeof o === "string" ? o : JSON.stringify(o);
    }
    let response = {
      timestamp: new Date(),
      lines: message.split("\\n"),
      isError,
    };
    setResponses([...responses, response]);
  }

  async function _importModule() {
    try {
      await importModule(modulePath);
      write(null);
    } catch (err: any) {
      write(err, true);
    }
  }

  async function _callFunction() {
    try {
      let response = await callFunction(moduleName, functionName, []);
      write(response);
    } catch (err: any) {
      write(err, true);
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
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          _callFunction();
        }}
      >
        <input
          id="module-name-input"
          onChange={(e) => setModuleName(e.currentTarget.value)}
          placeholder="Enter a module name..."
        />
        <input
          id="function-name-input"
          onChange={(e) => setFunctionName(e.currentTarget.value)}
          placeholder="Enter a function name..."
        />
        <button type="submit">Call function</button>
      </form>

      <div>
        {responses.map((s, i) => (
          <div key={i} className={s.isError ? "error" : ""}>
            <p>
              [{s.timestamp.toLocaleTimeString()}]{" "}
              {s.lines.length > 0 && s.lines[0]}
            </p>
            {s.lines.slice(1).map((line, j) => (
              <p key={j} className="other">
                {line}
              </p>
            ))}
          </div>
        ))}
      </div>
    </div>
  );
}

export default App;
