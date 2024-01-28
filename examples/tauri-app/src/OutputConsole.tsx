import { useEffect, useRef } from "react";
import "./OutputConsole.css";

interface Output {
  timestamp: Date;
  lines: string[];
  isError: boolean;
}

interface OutputConsoleProps {
  outputs: Output[];
}

function OutputConsole({ outputs }: OutputConsoleProps) {
  const ref = useRef<HTMLDivElement | null>(null);
  useEffect(() => {
    ref.current?.lastElementChild?.scrollIntoView();
  }, [outputs]);
  return (
    <div className="OutputConsole" ref={ref}>
      {outputs.map((output, idx) => (
        <div key={idx} className={output.isError ? "error" : ""}>
          {output.lines.map((line, i) => (
            <p key={i}>
              {i === 0
                ? `[${output.timestamp.toLocaleTimeString()}] `
                : "           "}
              {line}
            </p>
          ))}
        </div>
      ))}
    </div>
  );
}

export default OutputConsole;
