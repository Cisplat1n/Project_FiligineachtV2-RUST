# Quartet-Based Phylogenetic Network Inference (Rust)

High-performance quartet analysis for detecting reticulation events in phylogenetic trees.

## Quick Start

### System Architecture
pub enum RootingStrategy {
    KnownOutgroup(String),              // Use if available
    MAD,                                // Good default
    SpeciesTreeReconciliation(Tree),    // Best if species tree known
    Discordance,                        // Fallback only
}

pub fn root_quartet(
    quartet: &[&str; 4],
    gene_trees: &[Tree],
    strategy: RootingStrategy,
) -> String {
    match strategy {
        RootingStrategy::KnownOutgroup(ref og) if quartet.contains(&og.as_str()) => {
            og.clone()
        }
        RootingStrategy::MAD => {
            root_by_mad(quartet, gene_trees)
        }
        RootingStrategy::SpeciesTreeReconciliation(ref sp_tree) => {
            root_by_species_tree_reconciliation(quartet, gene_trees, sp_tree)
        }
        RootingStrategy::Discordance => {
            root_by_discordance(quartet, gene_trees)
        }
        _ => {
            // Known outgroup not in quartet - fall back to MAD
            root_by_mad(quartet, gene_trees)
        }
    }
}
```

---

## 🦀 **Complete Rust Implementation**

### **Project Structure**
```
quartet-rs/
│
├── Cargo.toml
├── benches/
│   └── benchmarks.rs
│
├── src/
│   ├── lib.rs                  # Public API
│   ├── main.rs                 # Binary entry point
│   │
│   ├── tree/
│   │   ├── mod.rs
│   │   ├── parser.rs           # Newick parser
│   │   ├── structure.rs        # Tree data structure
│   │   └── operations.rs       # Prune, root, distance
│   │
│   ├── quartet/
│   │   ├── mod.rs
│   │   ├── extractor.rs        # Generate quartets
│   │   ├── topology.rs         # Extract topology
│   │   └── methods.rs          # Distance/Parsimony/ML
│   │
│   ├── resolution/
│   │   ├── mod.rs
│   │   ├── threshold.rs        # Threshold resolution
│   │   ├── cycle.rs            # Cycle detection
│   │   └── delta.rs            # Delta calculations
│   │
│   ├── rooting/
│   │   ├── mod.rs
│   │   ├── mad.rs              # MAD rooting
│   │   ├── reconciliation.rs   # Species tree reconciliation
│   │   └── discordance.rs      # Discordance-based
│   │
│   ├── export/
│   │   ├── mod.rs
│   │   └── squirrel.rs         # SQUIRREL format export
│   │
│   └── utils/
│       ├── mod.rs
│       ├── memory.rs           # Memory management
│       └── parallel.rs         # Parallel utilities
│
└── python/
    ├── pyproject.toml
    ├── src/
    │   └── lib.rs              # PyO3 bindings (minimal)
    └── visualize.py            # PhySquirrel wrapper

### Build



### Run


### With Visualization


## Requirements


## Configuration


## Features

- Streaming tree processing (constant memory)
- Parallel quartet extraction and resolution
- SIMD-optimized cycle detection
- Multiple rooting strategies (MAD, outgroup, reconciliation)
- Lock-free data aggregation
- Zero-copy data passing

## Output

Generates SQUIRREL format file ready for PhySquirrel network visualization.
