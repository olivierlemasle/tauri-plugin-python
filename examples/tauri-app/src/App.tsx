import { useState } from "react";
import {
  importModule,
  callFunction,
  addResourcePathToSysPath,
} from "tauri-plugin-python-api";
import ArgInput from "./ArgInput";
import OutputConsole from "./OutputConsole";
import "./App.css";

interface Response {
  timestamp: Date;
  lines: string[];
  isError: boolean;
}

interface Arg {
  id: number;
  value: any;
  valid: boolean;
}

let nextId = 0;

function App() {
  const [path, setPath] = useState("");
  const [moduleName, setModuleName] = useState("");
  const [functionName, setFunctionName] = useState("");
  const [args, setArgs] = useState<Arg[]>([]);
  const [outputs, setOutputs] = useState<Response[]>([]);

  function write(o: any, isError = false) {
    let message = "OK";
    if (o) {
      message = typeof o === "string" ? o : JSON.stringify(o);
    }
    let output = {
      timestamp: new Date(),
      lines: message.split("\\n"),
      isError,
    };
    setOutputs([...outputs, output]);
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
      let posargs = args.map((e) => e.value);
      let response = await callFunction(moduleName, functionName, posargs);
      write(response);
    } catch (err: any) {
      write(err, true);
    }
  }

  return (
    <div className="App">
      <div className="panel">
        <h1>tauri-plugin-python example</h1>
        <h2>Sys path</h2>
        <form
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

        <h2>Module name</h2>
        <form
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

        <h2>Function</h2>
        <form
          onSubmit={(e) => {
            e.preventDefault();
            _callFunction();
          }}
        >
          <input
            id="function-name-input"
            onChange={(e) => setFunctionName(e.currentTarget.value)}
            placeholder="Enter a function name..."
          />
          <button className="primary" type="submit">
            Call function
          </button>
          <div>
            {args.map((arg, n) => (
              <div key={arg.id} className="row">
                <ArgInput
                  n={n}
                  valid={arg.valid}
                  defaultValue={arg.value}
                  onChange={(valid, value) => {
                    setArgs(
                      args.map((e) =>
                        e.id === arg.id
                          ? { ...e, valid: valid, value: value }
                          : e
                      )
                    );
                  }}
                  onRemove={() => {
                    setArgs(args.filter((e) => e.id !== arg.id));
                  }}
                />
              </div>
            ))}
            <button
              onClick={(e) => {
                e.preventDefault();
                setArgs([...args, { id: nextId++, valid: true, value: null }]);
              }}
            >
              Add argument
            </button>
          </div>
        </form>
      </div>
      <OutputConsole outputs={outputs} />
    </div>
  );
}

export default App;
