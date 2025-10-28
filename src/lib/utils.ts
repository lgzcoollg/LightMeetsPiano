import type { ClassValue } from "clsx"
import { clsx } from "clsx"
import { twMerge } from "tailwind-merge"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

// Detect if running inside a Tauri Webview window
export function isTauri(): boolean {
  return typeof window !== "undefined" &&
    // __TAURI_INTERNALS__ is injected by the Tauri runtime
    // Use the `in` operator directly on Window without unsafe casting
    ("__TAURI_INTERNALS__" in window);
}
