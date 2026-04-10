import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

export function useCounter() {
  const count = ref(0);
  const loading = ref(false);

  async function invokeCounter(command: string): Promise<void> {
    loading.value = true;
    try {
      count.value = await invoke<number>(command);
    } finally {
      loading.value = false;
    }
  }

  async function increment() {
    await invokeCounter("increment_tray_counter");
  }

  async function decrement() {
    await invokeCounter("decrement_tray_counter");
  }

  async function reset() {
    await invokeCounter("reset_tray_counter");
  }

  return {
    count,
    loading,
    increment,
    decrement,
    reset,
  };
}
