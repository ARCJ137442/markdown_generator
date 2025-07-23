# Markdown Generator

A Rust command-line tool that generates markdown documentation from source code files by traversing directory structures.

## Features

- Converts source code files to markdown code blocks with proper syntax highlighting
- Preserves directory structure in the output
- Supports a wide range of programming languages (see below)
- Can be configured to include all files or only supported ones

## Installation

1. Install Rust: <https://www.rust-lang.org/tools/install>
2. Clone this repository
3. Build the project:

```bash
cargo build --release
```

## Usage

Copy the executable to a directory in the root of your project, and run:

```bash
# Generate markdown for supported file types only for current directory
./markdown_generator

# Generate markdown for ALL files (including unsupported types) for current directory
./markdown_generator --all
```

The output will be saved to `output.md` in the current directory.

## Supported Languages

| Extension | Language      |
|-----------|---------------|
| md        | markdown      |
| json      | json          |
| yml/yaml  | yaml          |
| toml      | toml          |
| ini       | ini           |
| txt       | plaintext     |
| xml       | xml           |
| py        | python        |
| c         | c             |
| h/hpp/cpp | cpp           |
| java      | java          |
| rs        | rust          |
| kt        | kotlin        |
| jl        | julia         |
| sh/bash   | bash          |
| cmd/bat   | cmd           |
| ps1       | powershell    |
| lua       | lua           |
| cs        | csharp        |
| ts/tsx    | typescript    |
| js/jsx    | javascript    |
| go        | go            |
| zig       | zig           |
| nim       | nim           |
| gleam     | gleam         |
| ex/exs    | elixir        |
| erl/hrl   | erlang        |
| hs/lhs    | haskell       |
| ml/mli    | ocaml         |
| pl/pm     | perl          |
| php       | php           |
| rb/erb    | ruby          |
| r         | r             |
| rst       | rst           |
| sql       | sql           |
| swift     | swift         |
| vue       | vue           |
| css/scss  | css/scss      |
| less      | less          |

## Example Output

````markdown
# src

# src/main.rs

````rust
use ignore::Walk;
use phf::phf_map;
use std::ffi::OsStr;
...
````y


## License

MIT
