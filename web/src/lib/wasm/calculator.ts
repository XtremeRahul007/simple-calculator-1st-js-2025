import init, { greet, inspect_text, calculate, type CalculationResponse } from './pkg/calculator_wasm';

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

export type Operation = 'divide' | 'multiply';

export async function calculateExpression(left: string, right: string, operation: Operation): Promise<CalculationResponse> {
  await initializeWasm();

  return calculate(left, right, operation);
}
