# Quartet-Based Phylogenetic Network Inference (Rust)

High-performance quartet analysis for detecting reticulation events in phylogenetic trees.

## Quick Start

### System Architecture


---

## 🦀 **Complete Rust Implementation**

### **Project Structure**

```text
quartet-rs/
│
├── Cargo.toml
│
├── src/
│   ├── lib.rs                  # Public API
│   ├── main.rs                 # Binary entry point
│   │
│   ├── tree/                   ✅COMPLETE FOR NOW! 
│   │   ├── mod.rs
│   │   ├── parser.rs           # Newick parser
│   │   ├── structure.rs        # Tree data structure    
│   │   └── operations.rs       # Prune, root, distance
│   │
│   ├── quartet/
│   │   ├── mod.rs
│   │   ├── extractor.rs        # Generate quartets
│   │   ├── export.rs           # Extract topology
│   │   ├── aggregrate.rs       # 
│   │   ├── classify.rs         #   
│   │   └── root.rs             # 
│   │
│   │
│   └── utils/
│       ├── mod.rs
│       ├── memory.rs           # Memory management
│       └── parallel.rs         # Parallel utilities
│
├── tests/
│   ├── tree_stage.rs
│   ├── quartet_stage.rs
│   ├── resolution_stage.rs
│   └── e2e_pipeline.rs
│
└── python/
    ├── pyproject.toml
    ├── Cargo.toml
    ├── src/
        └── lib.rs              # PyO3 bindings (minimal)
```

### Build



### Run


### With Visualization


## Requirements


## Configuration


## Features

- Streaming tree processing (constant memory)
- Parallel quartet extraction and resolution
- Zero-copy data passing

## Output

Generates SQUIRREL format file ready for PhySquirrel network visualization.
