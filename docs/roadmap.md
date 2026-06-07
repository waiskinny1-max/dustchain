# Roadmap

| Version | Target |
|---|---|
| v0.1 | Core local chain |
| v0.2 | Fee engine and mempool ordering |
| v0.3 | Binary format and inspectors |
| v0.4 | Persistent storage with embedded DB |
| v0.5 | Local P2P networking |
| v0.6 | Live terminal UI |
| v0.7 | Benchmark suite |
| v0.8 | Local adversarial lab |
| v1.0 | Portfolio release |

Build order:

```text
core types -> hashing -> keys -> signatures -> transactions -> fees -> state transition -> block creation -> validation -> CLI -> binary encoding -> storage -> mempool -> benchmarks -> local networking -> TUI -> lab -> docs
```
