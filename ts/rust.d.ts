declare module '*.rs' {
  const mod: (options: any) => Promise<WebAssembly.ResultObject>;
  export default mod;
}
declare module '*.wasm' {
  const mod: (options: any) => Promise<WebAssembly.ResultObject>;
  export default mod;
}
