/// No error.
pub const ERROR_NONE: i8 = 0;

/// Error code of syntax error.
pub const ERROR_UNEXPECTED_CLOSER: i8 = -1;

/// Error code of syntax error.
pub const ERROR_UNCLOSED_LOOP: i8 = -2;

/// Error code of memory out of range.
pub const ERROR_MEMORY_OUT_OF_RANGE: isize = -10;

/// Error code of program ended.
pub const ERROR_ENDED: isize = -11;

/// Error code of program waiting for input.
pub const ERROR_WAITING_INPUT: isize = -12;
