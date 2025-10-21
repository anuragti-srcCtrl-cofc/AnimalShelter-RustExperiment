// error.rs

use std::path::PathBuf;
use thiserror::Error;

/// A centralized application error type.
#[derive(Error, Debug)]
pub enum AppError {
	#[error("File I/O Error: {0}")]
	Io(#[from] std::io::Error),

	#[error("Input file not found: {0}")]
	InputFileNotFound(PathBuf),

	#[error("Failed to create output directory: {0}")]
	OutputDirectoryCreate(PathBuf),

	#[error("Parsing error on line: {0}")]
	LineParseError(String),
	
	#[error("Invalid animal type character: '{0}'. Must be D, C, B, H, or R.")]
	InvalidAnimalType(char),

	#[error("Invalid registration number: '{0}' is not a valid u64.")]
	InvalidRegNum(String),

	#[error("Malformed line: expected {expected} fields, got {got} on line: {line}")]
	MalformedLine {
		expected: usize,
		got: usize,
		line: String,
	},

	#[error("A concurrent task failed to execute: {0}")]
	TaskJoin(#[from] tokio::task::JoinError),
}

/// A custom Result type alias for our application.
pub type AppResult<T> = Result<T, AppError>;
