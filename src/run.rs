use super::program::*;
use super::error::*;
#[allow(unused_imports)]
use js_debug;

/// Parse a BF program.
pub fn parse(code: &[u8]) -> Result<Vec<Op>, i8> {
  let mut result: Vec<Op> = vec![];
  // stack of loop openers.
  let mut stack: Vec<usize> = vec![];

  let mut idx = 0usize;
  for &op in code {
    if op == OP_PLUS {
      result.push(Op::Plus(1));
      idx += 1;
    } else if op == OP_MINUS {
      result.push(Op::Minus(1));
      idx += 1;
    } else if op == OP_LEFT {
      result.push(Op::Left(1));
      idx += 1;
    } else if op == OP_RIGHT {
      result.push(Op::Right(1));
      idx += 1;
    } else if op == OP_OUT {
      result.push(Op::Out);
      idx += 1;
    } else if op == OP_IN {
      result.push(Op::In);
      idx += 1;
    } else if op == OP_OPEN {
      stack.push(idx);
      result.push(Op::Open(0usize));
      idx += 1;
    } else if op == OP_CLOSE {
      if let Some(addr) = stack.pop() {
        result.push(Op::Close(addr));
        result[addr] = Op::Open(idx);
        idx += 1;
      } else {
        return Err(ERROR_UNEXPECTED_CLOSER);
      }
    }
    // ignore other characters.
  }
  if stack.len() > 0 {
    // !?
    return Err(ERROR_UNCLOSED_LOOP);
  }
  Ok(result)
}

pub struct RunResult {
  /// read input.
  pub input_read: usize,
  /// output size.
  pub output_size: usize,
  /// next pc.
  pub pc: usize,
  /// next ptr.
  pub ptr: usize,
  /// error code.
  pub error: Option<isize>,
}

/// Run a BF program.
pub fn run(
  // input buffer.
  inbuf: &[u8],
  // output buffer.
  outbuf: &mut [u8],
  // code.
  code: &[Op],
  // memory.
  memory: &mut [u8],
  // pc.
  mut pc: usize,
  // memory pointer.
  mut ptr: usize,
  ) -> RunResult {

    let mut in_bytes = 0;
    let mut written_bytes = 0;

    while pc < code.len() {
      // fetch operator.
      let op = code[pc];
      match op {
        Op::Plus(v) => {
          memory[ptr] = memory[ptr].wrapping_add(v);
        },
        Op::Minus(v) => {
          memory[ptr] = memory[ptr].wrapping_sub(v);
        },
        Op::Left(v) => {
          if ptr < v {
            return RunResult {
              input_read: in_bytes,
              output_size: written_bytes,
              pc,
              ptr,
              error: Some(ERROR_MEMORY_OUT_OF_RANGE),
            };
          }
          ptr = ptr - v;
        },
        Op::Right(v) => {
          if ptr + v >= memory.len() {
            return RunResult {
              input_read: in_bytes,
              output_size: written_bytes,
              pc,
              ptr,
              error: Some(ERROR_MEMORY_OUT_OF_RANGE),
            };
          }
          ptr = ptr + v;
        },
        Op::Out => {
          if written_bytes < outbuf.len() {
            outbuf[written_bytes] = memory[ptr];
            written_bytes += 1;
          } else {
            // cannot write!
            return RunResult {
              input_read: in_bytes,
              output_size: written_bytes,
              pc,
              ptr,
              error: None,
            };
          }
        },
        Op::In => {
          if in_bytes < inbuf.len() {
            // read from in memory
            memory[ptr] = inbuf[in_bytes];
            in_bytes += 1;
          } else {
            // nothing to read
            return RunResult {
              input_read: in_bytes,
              output_size: written_bytes,
              pc,
              ptr,
              error: Some(ERROR_WAITING_INPUT),
            };
          }
        },
        Op::Open(j) => {
          if memory[ptr] == 0 {
            // jump!
            pc = j + 1;
            continue;
          }
        },
        Op::Close(j) => {
          // short-cut
          if memory[ptr] != 0 {
            pc = j + 1;
            continue;
          }
        },
      }
      pc += 1;
    }
    // Program ended
    return RunResult {
      input_read: in_bytes,
      output_size: written_bytes,
      pc,
      ptr,
      error: Some(ERROR_ENDED),
    };
  }

