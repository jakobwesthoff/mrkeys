import { useEffect, useRef, useState } from "react";
import { Channel, invoke } from "@tauri-apps/api/core";
import "./App.css";

type KeygrabberReference = {
  id: string | null,
  is_loading: boolean
};

type Event = any;

function App() {
  const is_mounted_ref = useRef<boolean>(false);
  const keygrabber_ref = useRef<KeygrabberReference>({ id: null, is_loading: false });
  const [lastEvents, setLastEvents] = useState<Event[]>([]);
  const MAX_EVENTS = 8;

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
        channel.onmessage = (event: any) => {
          if (!event || !event.event_type || !event.event_type.KeyPress) {
            return;
          }
          setLastEvents((events) => {
            return [...events.slice(Math.max(0, events.length - (MAX_EVENTS - 1))), event];
          });
        };
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
    <main className="bg-zinc-900 text-zinc-100">
      <div className="justify-end flex">
        {lastEvents.map(event => {
          return <>
            <div className="min-w-[80px] h-[60px] border-amber-400 border flex m-1" >
              <div className="w-full h-full flex items-center justify-center text-3xl p-2">
                {event.event_type.KeyPress}
              </div>
            </div>
          </>
        })}
      </div>
    </main>
  );
}

export default App;
