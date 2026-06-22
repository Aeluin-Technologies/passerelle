# 🌉 passerelle  

SSoT for all shared interfaces, data structures, and inter-communication contracts across the Aeluin Technologies ecosystem.

It decouples the core analytical intelligence (**Galadril**) from the autonomous tactical hardware swarms (**Ringil**), while providing the deterministic deployment constraints used by the **Artemis** (NixOS/Kubernetes) control plane.

## Language Integration

This repository automatically compiles and distributes native code stubs to eliminate manual serialization code across our multi-language stack:

### Rust

```toml
[dependencies]
passerelle = { git = "https://github.com/Aeluin-Technologies/passerelle.git", branch = "main" }
```

### Python

```bash
uv add https://github.com/Aeluin-Technologies/passerelle.git
```

### Go

```go
import "https://github.com/Aeluin-Technologies/passerelle/go/interop/v1"
```
