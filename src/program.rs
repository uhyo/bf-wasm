/// AST of BF source.

pub const OP_PLUS: u8 = '+' as u8;
pub const OP_MINUS: u8 = '-' as u8;
pub const OP_LEFT: u8 = '<' as u8;
pub const OP_RIGHT: u8 = '>' as u8;
pub const OP_OUT: u8 = '.' as u8;
pub const OP_IN: u8 = ',' as u8;
pub const OP_OPEN: u8 = '[' as u8;
pub const OP_CLOSE: u8 = ']' as u8;

/// Vertual operator.
#[derive(Copy, Clone)]
pub enum Op {
  Plus(u8),
  Minus(u8),
  Left(usize),
  Right(usize),
  Out,
  In,
  /// Open which knows corresponding pc.
  Open(usize),
  /// Close which knows corresponding pc.
  Close(usize),
}
