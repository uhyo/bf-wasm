import loadWasm from '../src/lib.rs';

export async function test() {
  const mod = await loadWasm();
  console.log(mod.instance.exports.hi());
}
