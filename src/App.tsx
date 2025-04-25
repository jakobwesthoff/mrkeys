import { useEffect, useRef, useState } from "react";
import { Channel, invoke } from "@tauri-apps/api/core";
import "./App.css";

type KeygrabberReference = {
  id: string | null,
  is_loading: boolean
};

function App() {
  const is_mounted_ref = useRef<boolean>(false);
  const keygrabber_ref = useRef<KeygrabberReference>({ id: null, is_loading: false });
  const [lastEvent, setLastEvent] = useState<any>(null);

  useEffect(() => {
    is_mounted_ref.current = true;

    return () => { is_mounted_ref.current = false };
  }, []);

  // Register keygrabber channel on mount
  useEffect(() => {
    const cleanup = () => {
      const id = keygrabber_ref.current.id;
      keygrabber_ref.current.id = null;
      // ASYNC: We do not care about possible races with other registrations,
      // as we set the id to null immediately and then fire and forget.
      (async () => {
        if (keygrabber_ref.current.is_loading === false && id !== null) {
          console.log(`Unregister keygrabber: ${id}`);
          await invoke("unregister_keygrabber", { id });
        }
      })();
    }


    if (!keygrabber_ref.current.is_loading) {
      keygrabber_ref.current.is_loading = true;
      // ASYNC: Handling this async is okay, as we are checking for
      // is_mounted_ref and handling cleanup manually, after the await, should
      // the component been unmounted in the meantime.
      (async () => {
        const channel = new Channel<unknown>();
        channel.onmessage = (event: unknown) => {
          setLastEvent(event)
        }
        const id = await invoke<string>("register_keygrabber", { channel });
        console.log(`Registered keygrabber: ${id}`);
        keygrabber_ref.current.is_loading = false;
        keygrabber_ref.current.id = id;

        if (!is_mounted_ref.current) {
          cleanup();
        }
      })();
    }

    return cleanup;
  }, []);

  return (
    <main className="container">
      <div className="row">
        <pre style={{ textAlign: "left" }}>{JSON.stringify(lastEvent, undefined, 2)}</pre>
      </div>
    </main>
  );
}

export default App;
