import { Button, Input } from "@/components";
import { useInitialData } from "@/hooks";

import { emit } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import React, { useEffect, useRef } from "react";

const App: React.FC = ({}) => {
  const data = useInitialData();
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    invoke("close_splashscreen");
  }, []);

  const submitHandler = async (action: "set" | "add") => {
    const input = inputRef.current;

    if (input) {
      const value = input.valueAsNumber;
      if (value == null || value < 0) return;
      await emit("events::souls", { action, value });
    }
  };

  return (
    <main className="flex flex-col bg-neutral-900 h-screen justify-between text-white p-4">
      <div className="w-full space-y-2">
        <Input
          type="number"
          min={0}
          max={9_999_999}
          placeholder={"souls amount"}
          ref={inputRef}
        />
        <div className="w-full inline-flex gap-x-2">
          <Button className="w-full" onClick={() => void submitHandler("add")}>
            add
          </Button>
          <Button className="w-full" onClick={() => void submitHandler("set")}>
            set
          </Button>
        </div>
      </div>
      <footer className="w-full inline-flex justify-between text-neutral-400">
        <div className="text-xs">
          <p>
            <span>PHANDLE</span>
            <span>({data?.phandle})</span>
          </p>
          <p>
            <span>PID</span>
            <span>({data?.pid})</span>
          </p>
        </div>
        <a href="" className="outline-none font-mono hover:underline">
          Txreq 🐔
        </a>
      </footer>
    </main>
  );
};

export default App;
