declare module '*.rs' {
  const mod: () => Promise<WebAssembly.ResultObject>;
  export default mod;
}
