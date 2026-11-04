import init, { greet } from './pkg/calculator_wasm'

let wasmInit: ReturnType<typeof init> | undefined

export async function greetFromRust(name: string): Promise<string> {
  wasmInit ??= init();

  await wasmInit;

  return greet(name);
}
