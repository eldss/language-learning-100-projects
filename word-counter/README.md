# Word Counter (Rust)

## 1. Overview

The Word Counter project is a command-line utility written in Rust that analyzes text files or standard input to count words, characters, and lines. It is designed to help strengthen understanding of Rust’s standard library, error handling, and command-line argument parsing.

## 2. Purpose

This project’s purpose is to build familiarity with:

- File and standard input handling in Rust.
- String and text processing.
- Implementing command-line interfaces.
- Safe and idiomatic error handling.

## 3. Scope

### 3.1 In Scope

- Reading input from a file or standard input.
- Counting:
  - Words.
  - Characters (with and without whitespace).
  - Lines.
- Customizable output through command-line flags.
- Graceful error handling for I/O and decoding issues.

### 3.2 Out of Scope

- Graphical user interfaces.
- Networked or remote file inputs.
- Parallel or asynchronous processing (optional future enhancement).

## 4. Functional Requirements

### 4.1 Input Handling

- Accept a file path as a command-line argument.
- If no argument is provided, read text from standard input.

### 4.2 Counting Logic

- Words: split text by whitespace.
- Lines: count newline characters.
- Characters: count UTF-8 codepoints.

### 4.3 Output

- Default output:

```txt
Lines: 42  
Words: 358  
Characters: 1790  
```

- Optional flags:
  - `-l` or `--lines`: show only line count.  
  - `-w` or `--words`: show only word count.  
  - `-c` or `--chars`: show only character count.

### 4.4 Error Handling

- Display clear messages for invalid file paths or read errors.
- Handle non-UTF-8 input safely with fallbacks or warnings.

## 5. Non-Functional Requirements

- Must compile cleanly using `cargo build` without warnings.
- Follows idiomatic Rust practices using `Result`, `Option`, and the `?` operator.
- Includes at least one unit test for each counting function.
- Uses only the Rust standard library; external crates are optional but minimal.

## 6. Optional Extensions

- Support multiple file inputs.
- Implement multithreading for large files.
- Add top-N word frequency analysis.
- Output results in JSON or CSV format.

## 7. Success Criteria

- Correct output for varied text inputs.
- All tests pass using `cargo test`.
- Handles all I/O errors without panicking.
- Output remains consistent, clear, and user-friendly.
