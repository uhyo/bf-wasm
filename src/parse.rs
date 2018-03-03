use super::program::*;

/// Parse BF program.
pub fn parse(code: Vec<u8>) -> Result<Block, &'static str> {
  let mut it = code.into_iter();
  let mut program = ProgramBlock {
    blocks: vec![],
  };
  let _ = parse_blocks(it, &mut program.blocks);

}

/// Parse into blocks.
pub fn parse_blocks(code: &mut(impl Iterator<Item=u8>), blocks: &mut Vec<Block>) -> Result<(), &'static str> {
  let mut ops: Vec<u8> = vec![];
  for op in code {
    if op == OP_OPEN {
      // Enter a new loop.
      if ops.len() > 0 {
        blocks.push(Block::new_ops(ops));
        ops = vec![];
      }
      let mut lbls = vec![];
      parse_loop(code
    }
  }
}
