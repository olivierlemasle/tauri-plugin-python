import { useState } from "react";
import {
  importModule,
  callFunction,
  addResourcePathToSysPath,
} from "tauri-plugin-python-api";
import "./App.css";

interface Response {
  timestamp: Date;
  lines: string[];
  isError: boolean;
}

function App() {
  const [path, setPath] = useState("");
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

  async function _addResourcePathToSysPath() {
    try {
      await addResourcePathToSysPath(path);
      write(null);
    } catch (err: any) {
      write(err, true);
    }
  }

  async function _importModule() {
    try {
      await importModule(moduleName);
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
          _addResourcePathToSysPath();
        }}
      >
        <input
          id="module-path-input"
          onChange={(e) => setPath(e.currentTarget.value)}
          placeholder="Enter a path..."
        />
        <button type="submit">Add to sys path</button>
      </form>
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          _importModule();
        }}
      >
        <input
          id="module-name-input"
          onChange={(e) => setModuleName(e.currentTarget.value)}
          placeholder="Enter a module name..."
        />
        <button type="submit">Import module</button>
      </form>
      <form
        onSubmit={(e) => {
          e.preventDefault();
          _callFunction();
        }}
      >
        <div>
          <input
            id="function-name-input"
            onChange={(e) => setFunctionName(e.currentTarget.value)}
            placeholder="Enter a function name..."
          />
          <button type="submit">Call function</button>
        </div>
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
