import { invoke } from "@tauri-apps/api/core";

export function setBadgeCount(count: number): Promise<void> {
  return invoke("set_badge_count", { count });
}

export function pinWindow(): Promise<void> {
  return invoke("pin_window");
}

export function unpinWindow(): Promise<void> {
  return invoke("unpin_window");
}

export function hideWindow(): Promise<void> {
  return invoke("cmd_front_hide");
}
