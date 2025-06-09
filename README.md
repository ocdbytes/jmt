# Jellyfish Merkle Tree (JMT)

> [!WARNING]
> DON'T USE THE MULTIPROOF IMPLEMENTATION IN PROD.
> It is not tested and audited properly yet. After updating
> the test suite with all cases it can be used in dev environment.

JMT is a highly efficient and append-only authenticated data structure designed to support versioned key-value stores. It is primarily used in blockchain systems like Diem for proving state inclusions and exclusions efficiently.

This implementation supports:

* Sparse Merkle Tree (SMT) with 256-bit keys
* Efficient incremental updates
* Support for versioning
* Proof generation for single keys and **batch proofs for multiple keys**

---

## Batch Sparse Merkle Proofs in JMT

This section explains how batch proofs work in the Jellyfish Merkle Tree (JMT) implementation — from generation to verification.

### What Is a Batch Sparse Merkle Proof?

A **BatchSparseMerkleProof** allows proving multiple key/value pairs in a compact format by:

* Storing **leaf nodes** for all queried keys
* Sharing **common sibling nodes** across Merkle paths
* Using index-based lookups to reconstruct the root efficiently

This dramatically reduces proof size compared to individual proofs.

---

### Structure

```rust
pub struct BatchSparseMerkleProof<H: SimpleHasher> {
    pub leaves: Vec<Option<SparseMerkleLeafNode>>,   // proof leaf for each key
    pub shared_siblings: Vec<SparseMerkleNode>,      // deduplicated sibling set
    pub proof_paths: Vec<Vec<usize>>,                // sibling indices for each key
    _phantom: PhantomData<H>,
}
```

* `leaves[i]` matches the i-th key
* `proof_paths[i]` is a list of sibling indices into `shared_siblings` for `keys[i]`

---

### Example Output

```rust
BatchSparseMerkleProof {
  leaves: [
    Some(SparseMerkleLeafNode { key_hash: ..., value_hash: ... }),
    Some(SparseMerkleLeafNode { key_hash: ..., value_hash: ... }),
  ],
  shared_siblings: [
    Internal(...),     // reused
    Null,
    Leaf(...),
    Leaf(...),
  ],
  proof_paths: [
    [0, 1, 1, 1],     // for key 1
    [2, 3, 1, 1],     // for key 2
  ]
}
```

---

### Generating a Batch Proof

Use:

```rust
let (values, proof) = tree.get_batch_with_proof(vec![key1, key2], version)?;
```

The result:

* `values` contains the values of queried keys
* `proof` contains a `BatchSparseMerkleProof`

---

### Verifying a Batch Proof

Call:

```rust
proof.verify::<Sha256Hasher>(&[key1, key2], &values, expected_root)?;
```

This will:

1. Validate each key's inclusion or non-inclusion
2. Recompute each root from leaf to top using `proof_paths`
3. Confirm the final hash equals the known root

Returns `Ok(())` if all roots match, otherwise `Err`.

---

### How It Works Internally

For each key:

1. Calculate the leaf hash (or placeholder if non-existent)
2. Use `proof_paths[i]` and corresponding sibling hashes
3. Combine hashes using left/right logic via bit iteration
4. Resulting root must match `expected_root`

---

### Data Flow Summary

```text
       ┌───────────────────────────┐
       │ Keys: [k1, k2]     │
       └──────────────────────────┘
                 │
                 ▼
    ┌─────────────────────────────┐
    │ get_batch_with_proof(keys) │
    └────────────────────────────┘
                 │
                 ▼
       ┌──────────────────────────┐
       │ BatchSparseMerkle  │
       │ - leaves            │
       │ - shared_siblings   │
       │ - proof_paths       │
       └──────────────────────────┘
                │
                ▼
    ┌──────────────────────────────┐
    │ verify(keys, values, root) │
    └──────────────────────────────┘
```