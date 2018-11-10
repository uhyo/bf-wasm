# bf-wasm

Brainfuck interpreter written in WebAssembly (compiled from Rust).

# Requirements
- WebAssembly support
- `Symbol.asyncIterator` support

node.js v10 or later meets these requirements. Also, recent browsers like Chrome are fine.

# Usage

```js
const {
  BFInterpreter,
} = require('bf-wasm');

// BF source
const source = `
+++++++++[
>++++++++>+++++++++++
>++++++++++++>+++++<<<<-
]>.>++.>..+++.
>-.------------.<
++++++++.--------.
+++.------.<-.>>+.
`;

const inputIterator = process.stdin[Symbol.asyncIterator]();

// Function which reads user input
const getInput = ()=> inputIterator.next().then(({value})=> value != null ? value : '\x00');
  
// initialize the interpreter
const interpreter = new BFInterpreter(source, getInput);

(async ()=> {
  // Result of run() is an async iterator of chars
  for await (const char of interpreter.run()) {
    process.stdout.write(String.fromCharCode(char));
  }
  process.exit(0);
})();

```

# License
MIT

# Contribution
Welcome

