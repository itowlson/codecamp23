// Invoking a raw Wasm module

const runRaw = async () => {
  const moduleBytes = fetch("./raw-wasm-test/target/wasm32-unknown-unknown/release/raw_wasm_test.wasm");
  const { module, instance } = await WebAssembly.instantiateStreaming(moduleBytes);
  const addResult = instance.exports.add(234,567);
  document.body.textContent = `Called WebAssembly add function: result = ${addResult}`;
}

runRaw();

// Invoking using the wasm-pack helpers

// import init from "./wasm-test/pkg/wasm_test.js";

// const runPacked = async () => {
//   const wtest = await init();
//   const addResult = wtest.add(123, 456);
//   document.body.textContent = `Called WebAssembly add function: result = ${addResult}`;
// };

// runPacked();
