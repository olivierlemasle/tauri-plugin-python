import { useEffect, useRef } from "react";
import "./ArgInput.css";

interface ArgProps {
  n: number;
  valid: boolean;
  defaultValue?: any;
  onChange?: (valid: boolean, value?: any) => void;
  onRemove?: () => void;
}

function ArgInput({ n, valid, defaultValue, onChange, onRemove }: ArgProps) {
  const inputRef = useRef<HTMLInputElement | null>(null);

  useEffect(() => {
    inputRef.current?.focus();
  }, []);

  function update(content: string) {
    try {
      const s = content.trim();
      const o = s === "" ? null : JSON.parse(s);
      onChange?.(true, o);
    } catch (e: any) {
      onChange?.(false);
    }
  }

  return (
    <div>
      <input
        ref={inputRef}
        className={valid ? "valid" : "invalid"}
        onChange={(e) => {
          update(e.target.value);
        }}
        defaultValue={defaultValue}
        placeholder={"Arg " + n}
      />
      <button
        onClick={(e) => {
          e.preventDefault();
          onRemove?.();
        }}
      >
        Delete
      </button>
    </div>
  );
}

export default ArgInput;
