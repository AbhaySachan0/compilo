
# Compilo

**Compilo** is a simple C/C++ build system written in Rust.

The project is being built from scratch to understand how build systems work internally — from invoking compilers and generating object files to dependency tracking and incremental builds.

> **Status:** Early development — not production-ready yet.

---

## Why Compilo?

Build systems such as Make, CMake, Ninja, and Cargo automate many steps involved in building software.

Compilo is an attempt to understand those steps by implementing a small build system ourselves.

The current focus is:

- C/C++ compilation
- Object file generation
- Dependency tracking
- Incremental compilation
- Incremental linking
- Project initialization
- Configuration through `compilo.toml`

The project is intentionally kept simple and educational.

---

## Current Features

### Project initialization

Create a C++ project:

```bash
compilo init hello
```

Creates:

```text
hello/
├── compilo.toml
├── src/
│   └── main.cpp
└── build/
```

Create a C project:

```bash
compilo init --c hello
```

Creates:

```text
hello/
├── compilo.toml
├── src/
│   └── main.c
└── build/
```

---

### C and C++ support

Compilo currently recognizes:

| Extension | Compiler |
| --------- | -------- |
| `.c`      | `gcc`    |
| `.cpp`    | `g++`    |

For example:

```text
src/main.cpp
      │
      ▼
     g++
      │
      ▼
build/main.o
```

and:

```text
src/main.c
      │
      ▼
     gcc
      │
      ▼
build/main.o
```

---

## Configuration

Every Compilo project contains a `compilo.toml` file.

Example:

```toml
[project]
name = "hello"
source = "src"
build = "build"
```

The configuration currently specifies:

- Project name
- Source directory
- Build directory

Compilo uses this information when building the project.

---

## Building a Project

After creating a project:

```bash
compilo init hello
cd hello
compilo
```

Compilo:

1. Reads `compilo.toml`
2. Finds C/C++ source files
3. Determines the appropriate compiler
4. Compiles source files into object files
5. Generates dependency files
6. Checks whether recompilation is necessary
7. Links object files into the final executable

The resulting project looks approximately like:

```text
hello/
├── compilo.toml
├── src/
│   └── main.cpp
└── build/
    ├── main.o
    ├── main.d
    └── hello
```

---

# Incremental Compilation

One of the important features currently implemented is incremental compilation.

Suppose we have:

```text
src/
├── main.cpp
├── math.cpp
└── math.hpp
```

After the first build:

```text
main.cpp → main.o
math.cpp → math.o
```

If nothing has changed, Compilo does not compile everything again.

```text
Skipping "main.cpp"
Skipping "math.cpp"
```

If `math.cpp` changes:

```text
main.cpp → unchanged
math.cpp → recompile
```

This saves unnecessary compilation work.

---

# Dependency Tracking

Compilo uses GCC/G++ dependency generation:

```bash
-MMD
-MP
```

During compilation, the compiler generates a `.d` dependency file alongside the object file.

For example:

```text
build/
├── main.o
└── main.d
```

A dependency file can describe relationships such as:

```text
main.o: main.cpp math.hpp
```

Compilo reads this information to determine whether a source file needs to be rebuilt.

Therefore, changing a header such as:

```text
math.hpp
```

can cause the appropriate object file to be recompiled.

---

# Object Files

Compilo does not directly compile every source file into the final executable.

Instead, compilation happens in two stages.

### Compilation

```text
main.cpp
    │
    ▼
  g++ -c
    │
    ▼
 main.o
```

### Linking

```text
main.o
math.o
   │
   ▼
 linker
   │
   ▼
 hello
```

This separation is important because it allows Compilo to rebuild only the parts of a project that changed.

---

# Incremental Linking

Compilo also checks whether the executable needs to be linked again.

Conceptually:

```text
                ┌── main.o
                │
object files ───┼── math.o
                │
                └── utils.o
                       │
                       ▼
                     linker
                       │
                       ▼
                    program
```

If an object file is newer than the executable, Compilo performs linking again.

Otherwise:

```text
Skipping linking
```

---

# Current Architecture

The current implementation is intentionally simple.

```text
                     Compilo
                        │
                        ▼
                   CLI arguments
                        │
                        ▼
                  compilo.toml
                        │
                        ▼
                 Source discovery
                        │
                        ▼
                Compiler selection
                  ┌─────┴─────┐
                  │           │
                 gcc         g++
                  │           │
                  └─────┬─────┘
                        ▼
                  Object files
                        │
                        ▼
                 Dependency files
                        │
                        ▼
               Incremental checking
                        │
                        ▼
                     Linking
                        │
                        ▼
                    Executable
```

---

# Rust Concepts Used

Compilo is also being developed as a way to learn Rust through a real systems project.

The project currently uses concepts including:

### Filesystem operations

```rust
std::fs
```

Used for:

- Creating directories
- Reading files
- Writing files
- Reading directory contents
- Checking file metadata

### Paths

```rust
std::path::{Path, PathBuf}
```

Used to safely construct and manipulate filesystem paths.

### Process execution

```rust
std::process::Command
```

Used to execute external programs such as:

```text
gcc
g++
```

### Command-line arguments

```rust
std::env::args()
```

Used to implement commands such as:

```bash
compilo init hello
```

### Serialization / configuration

The project uses:

```text
serde
toml
```

to deserialize `compilo.toml` into Rust structures.

### Ownership and lifetimes

Dependency parsing also provided a practical example of Rust ownership.

For example, returning:

```rust
Vec<&str>
```

from a temporary `String` caused a lifetime error.

The implementation was changed to:

```rust
Vec<String>
```

so the returned values own their data.

---

# Current CLI

Currently supported:

```bash
compilo init <project-name>
```

Creates a C++ project.

```bash
compilo init --c <project-name>
```

Creates a C project.

```bash
compilo
```

Builds the current Compilo project.

---

# Example

Create a C++ project:

```bash
compilo init hello
```

Enter the project:

```bash
cd hello
```

Build it:

```bash
compilo
```

The executable will be generated in:

```text
build/hello
```

Run it:

```bash
./build/hello
```

Output:

```text
Hello from Compilo!
```

---

# Current Project Structure

The current implementation is still intentionally small.

```text
compilo/
├── src/
│   └── main.rs
├── Cargo.toml
└── Cargo.lock
```

As the project grows, the codebase will eventually be separated into smaller modules.

---

# Roadmap

The following features are planned, but are **not implemented yet**.

## Build configuration

- [ ] Compiler flags
- [ ] Include directories
- [ ] Library directories
- [ ] External libraries
- [ ] Debug/release build configurations

Example future configuration:

```toml
[build]
flags = ["-Wall", "-Wextra"]

[dependencies]
include = ["include"]
library_paths = []
libraries = []
```

---

## Source management

- [ ] Recursive source discovery
- [ ] Nested source directories
- [ ] Better dependency graph
- [ ] More robust dependency parsing

---

## Build system

- [x] Object file generation
- [x] Dependency file generation
- [x] Incremental compilation
- [x] Incremental linking
- [ ] Dependency graph
- [ ] Parallel compilation
- [ ] Better build scheduling

---

## CLI

Planned commands:

```bash
compilo init <name>
compilo
compilo run
compilo clean
compilo rebuild
compilo --help
compilo --version
```

---

## Quality

- [ ] Unit tests
- [ ] Integration tests
- [ ] Better error handling
- [ ] Cross-platform considerations
- [ ] Documentation improvements
- [ ] CI
- [ ] Release binaries

---

# Long-Term Goal

The long-term goal is to turn Compilo into a small but functional C/C++ build system while keeping its implementation understandable.

The project will explore concepts such as:

```text
Compiler invocation
       ↓
Object files
       ↓
Dependency tracking
       ↓
Incremental builds
       ↓
Linking
       ↓
Libraries
       ↓
Dependency graphs
       ↓
Parallel builds
       ↓
Build scheduling
```

The goal isn't to immediately compete with mature build systems.

The goal is to understand **how they work by building one from scratch**.

---

# Development

Clone the repository:

```bash
git clone <repository-url>
cd compilo
```

Build:

```bash
cargo build
```

Run directly with Cargo:

```bash
cargo run -- init hello
```

Build an existing project:

```bash
cd hello
/path/to/compilo
```

---

# Requirements

Currently Compilo requires:

- Rust
- GCC
- G++
- Linux/Unix-like environment

The project is currently being developed on Linux.

---

# License

License to be decided before the first public release.
