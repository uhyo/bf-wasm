// import loadWasm from '../src/lib.rs';
import loadWasm from '../target/wasm32-unknown-unknown/release/bf_gen_interpreter.wasm';

/**
 * BF interpreter class.
 */
export class BFInterpreter {
  /**
   * BF source.
   */
  protected source: string;
  /**
   * Source of input.
   */
  protected inputSource: () => Promise<string>;
  /**
   * Capacity of output buffer.
   */
  protected outCapacity = 2048;
  constructor(source: string, inputSource: () => Promise<string>) {
    this.source = source;
    this.inputSource = inputSource;
  }
  /**
   * Execute given BF program.
   */
  public async *run(): AsyncIterableIterator<number> {
    // convert code into u8.
    const code = Array.from(this.source, c => c.charCodeAt(0));
    // load Wasm.
    const mod = await loadWasm({
      global: {},
      env: {
        memory: new WebAssembly.Memory({ initial: 10, maximum: 100 }),
        table: new WebAssembly.Table({ initial: 0, element: 'anyfunc' }),
        js_debug: (num: number) => {
          console.debug('debug:', num);
        },
      },
    });
    const memory = mod.instance.exports.memory;
    const funcs = mod.instance.exports;
    // Request a buffer for source.
    const sourceBufferPtr = funcs.alloc(code.length);
    const sourceBuffer = new Uint8Array(
      memory.buffer,
      sourceBufferPtr,
      code.length,
    );
    // write code to the buffer.
    sourceBuffer.set(code);
    const parseResult = funcs.parse.call(null, sourceBufferPtr, code.length);
    if (parseResult === -1) {
      throw new Error('SyntaxError: unexpected loop closer');
    } else if (parseResult === -2) {
      throw new Error('SyntaxError: unclosed loop');
    } else if (parseResult !== 0) {
      throw new Error(`Error: unknown error ${parseResult}`);
    }

    // allocate output buffer.
    const outBufferPtr = funcs.alloc(this.outCapacity);
    const outBuffer = new Uint8Array(
      memory.buffer,
      outBufferPtr,
      this.outCapacity,
    );
    // Run!
    while (true) {
      console.log('running');
      const result = funcs.run(outBufferPtr, this.outCapacity);
      console.log('ran', result);
      if (result >= 0) {
        // output is generated.
        yield* new Uint8Array(outBuffer.buffer, outBuffer.byteOffset, result);
      } else if (result === -10) {
        throw new Error('Error: Memory out of range');
      } else if (result === -11) {
        // program ended!
        break;
      } else if (result === -12) {
        // program is waiting for input
        const input = await this.getInput();
        const inputBufferPtr = funcs.get_input_buf(input.length);
        const inputBuffer = new Uint8Array(
          memory.buffer,
          inputBufferPtr,
          input.length,
        );
        inputBuffer.set(input);
      } else {
        throw new Error(`Error: Unknown error ${result}`);
      }
    }
    // clean up
    funcs.free(outBufferPtr);
    funcs.free(sourceBufferPtr);
  }
  /**
   * Get a input from user.
   */
  protected async getInput(): Promise<number[]> {
    const buf = await this.inputSource();
    // TODO
    const nums = Array.from(buf, c => c.charCodeAt(0) % 256);
    return nums;
  }
}
