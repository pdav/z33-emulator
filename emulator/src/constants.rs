//! Various constants and type definitions used throughout the emulator

pub type Address = u32;
pub type Word = i64;

/// Total size of the computer memory
pub const MEMORY_SIZE: Address = 10_000;

/// Start of the stack pointer
pub const STACK_START: Address = MEMORY_SIZE;

/// Default place to store the beginning of the program
pub const PROGRAM_START: Address = 1000;

/// Address of the interrupt handler
pub const INTERRUPT_HANDLER: Address = 200;

/// Address where %pc is saved when an interruption occurs
pub const INTERRUPT_PC_SAVE: Address = 100;

/// Address where %sr is saved when an interruption occurs
pub const INTERRUPT_SR_SAVE: Address = 101;

/// Address the exception code is saved when an interruption occurs
pub const INTERRUPT_EXCEPTION: Address = 102;
