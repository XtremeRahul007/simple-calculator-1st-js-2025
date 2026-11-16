import init, { greet, inspect_text, calculate_division, type CalculationResponse } from './pkg/calculator_wasm';

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

export async function greetFromRust(name: string): Promise<{ greet: string; text: string; }> {
  await initializeWasm();

  return {
    greet: greet(name),
    text: inspect_text(name)
  };
}

export async function divide(left: string, right: string): Promise<CalculationResponse> {
  await initializeWasm();

  return calculate_division(left, right);
}
