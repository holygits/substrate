use core::fmt;
use parity_codec_derive::*;

/// A contract runtime error
#[derive(Decode, Encode)]
pub enum ContractError {
	// A contract execution exhausted its gas supply
	OutOfGas,
	// Something went wrong during contract execution
	Execution(Reason),
}

impl ContractError {
	/// Return a ContractError w.r.t a memory failure
	fn memory() -> Self {
		ContractError::Execution(Reason::InvalidMemoryAccess)
	}
	/// Return a ContractError w.r.t a codec failure
	fn codec() -> Self {
		ContractError::Execution(Reason::Codec)
	}
}

impl fmt::Display for ContractError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			OutOfGas => write!(f, "Out of gas"),
			Execution(reason) => write!(f, "During execution: {}", reason),
		}
	}
}

// Reason for execution failure
#[derive(Decode, Encode)]
enum Reason {
	// Contract ABI tried to decode a `T` and failed
	Codec(T),
	// Contract tried to access memory out of bounds
	InvalidMemoryAccess,
}

impl fmt::Display for Reason {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Reason::Codec => write!(f, "Type with bad encoding"),
			Reason::InvalidMemoryAccess => write!(f, "Invalid memory access"),
		}
	}
}
