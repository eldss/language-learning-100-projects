# 100 Projects to Learn a Language

This repository tracks my journey through the “100 Days of Rust Development” course while broadening the scope to include other languages when a project calls for it. The original curriculum is Rust-first: build one project per day, progressing from fundamentals to full-stack apps. I don't always get to one a day, I may add more as time goes on. Here, I'm adapting that pacing and project lineup to deepen my Rust skills and explore adjacent technologies.

Link to course: <https://www.udemy.com/course/rust-programming-bootcamp>

## Course Phases

| Days | Focus | Highlights |
| --- | --- | --- |
| 1–20 | Rust Fundamentals | Ownership, borrowing, control flow, small utilities |
| 21–40 | Intermediate Rust | Error handling, generics, concurrency, chat tools |
| 41–60 | Systems Programming | Filesystems, encryption, TCP/HTTP servers |
| 61–80 | Web Development | Actix-Web services, Yew frontends, authentication |
| 81–100 | Full-Stack Projects | End-to-end apps, real-time dashboards, capstone build |

I’ll pair the official project prompts with experiments in other languages as I see fit. Go is the next likely candidate.

## Repository Layout

- `project-name/` — one directory per project, named after its focus (`hello-world`, `temp-converter`, etc.) so I can re-order or regroup them freely.
- Language-specific tooling lives inside each project folder (e.g., a `Cargo.toml` for Rust, `package.json` for TypeScript, `pyproject.toml` for Python).
- Most projects will include a short write-up or notes on tradeoffs, gotchas, or cross-language comparisons.

## Getting Started

### Prerequisites

- [Rust toolchain](https://www.rust-lang.org/tools/install) for Rust-based projects (`cargo`, `rustc`).
- Additional language runtimes or build tools will be documented within each project folder.

### Running a Rust Project

```bash
cargo run --manifest-path hello-world/hello-rust/Cargo.toml
```

Use `cargo test` in the same folder when tests are available.
