import { invoke } from "@tauri-apps/api";
import { useEffect, useState } from "react";

export function useInitialData() {
  const [data, setData] = useState<InitialBackendData | undefined>(undefined);

  useEffect(() => {
    invoke("init_data").then((data) => {
      setData(data as InitialBackendData);
    });
  }, []);

  return data;
}
