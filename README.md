# sloc-rs

A fast, multi-threaded command-line tool written in Rust to count lines of code (SLOC) in your projects.

## Installation

### From Source

Ensure you have [Rust and Cargo](https://rustup.rs/) installed, then run:

```bash
git clone https://github.com/Emii-lia/sloc-rs.git
cd sloc-rs
cargo install --path .
```
Or install globally:

```bash
cargo install sloc-rs
```

## Usage

Run `sloc` followed by the directory you want to scan:

```bash
sloc <root_directory>
```

### Options

- `-t, --threads <THREADS>`: Specify the number of threads to use. Defaults to the number of logical cores available on your system.
- `-V, --version`: Print version information.
- `-h, --help`: Print help information.

### Example

```bash
$ sloc src
Lines of code: 77121
Blank lines: 6214
Files: 958
Elapsed time: 22ms
Done!
```
## How it Works

1. **File Collection**: It traverses the specified root directory, skipping ignored patterns and unsupported file types.
2. **Work Distribution**: Files are queued into a channel.
3. **Parallel Processing**: A pool of worker threads consumes the channel, reads file contents, and counts lines.
4. **Aggregation**: Results from all workers are merged and displayed.

## License

This project is licensed under the MIT License (or whichever license you prefer).
