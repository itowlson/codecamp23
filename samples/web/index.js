// Invoking a raw Wasm module

const runRaw = async () => {
  const moduleBytes = fetch("./raw-wasm-test/target/wasm32-unknown-unknown/release/raw_wasm_test.wasm");
  const { module, instance } = await WebAssembly.instantiateStreaming(moduleBytes);
  const addResult = instance.exports.add(234,567);
  document.body.textContent = `Called WebAssembly add function: result = ${addResult}`;
}

runRaw();

// Invoking using the wasm-pack helpers

// import init, { add } from "./wasm-test/pkg/wasm_test.js";

// const runPacked = async () => {
//   await init();
//   const addResult = add(123, 456);
//   document.body.textContent = `Called WebAssembly add function: result = ${addResult}`;
// };

// runPacked();
