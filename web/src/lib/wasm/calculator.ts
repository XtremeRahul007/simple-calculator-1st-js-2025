import init, { greet } from './pkg/calculator_wasm'

let wasmInit: ReturnType<typeof init> | undefined;

export async function initializeWasm(): Promise<void> {
  wasmInit ??= init();

  try {
    await wasmInit;
  } catch (err) {
    wasmInit = undefined;
    throw err;
  }
}

export async function greetFromRust(name: string): Promise<string> {
  await initializeWasm();

  return greet(name);
}
