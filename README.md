# AuraDB 🌀

An ultra-lightweight, edge-optimized vector database written in Rust. Designed for fast local semantic search on resource-constrained devices (IoT, Mobile, and Embedded Systems).

## Key Features 🚀

- **Sub-millimeter Footprint:** Runs comfortably on devices with < 64MB of RAM.
- **Quantization on the Fly:** Reduces vector dimensions natively with 70% memory savings.
- **Zero-Dependency Core:** Pure Rust implementation utilizing HNSW (Hierarchical Navigable Small World) graphs.
- **Local-First Privacy:** No data ever leaves the device. GDPR and HIPAA compliant by design.

## Quick Start 🛠️

Add AuraDB to your `Cargo.toml`:

```toml
[dependencies]
auradb = "0.1.0"
```

### Basic Usage

```rust
use auradb::{AuraEngine, Vector, Document};

fn main() {
    // Initialize the engine with 128-dimension embeddings
    let mut db = AuraEngine::new(128);

    // Insert a document with its vector embedding
    let doc = Document::new("doc_1", "Hello from AuraDB!");
    let embedding = vec![0.15; 128]; 
    
    db.insert(doc, Vector::from(embedding)).unwrap();

    // Query nearest neighbors
    let query_vector = Vector::from(vec![0.14; 128]);
    let results = db.query(&query_vector, 3).unwrap();

    println!("Found nearest neighbor: {:?}", results[0]);
}
