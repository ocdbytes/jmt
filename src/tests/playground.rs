use std::collections::HashMap;

use sha2::Sha256;

use crate::{
    mock::MockTreeStore, JellyfishMerkleTree, KeyHash, OwnedValue, TransparentHasher, Version,
};

#[test]
fn test_playground() {
    let db = MockTreeStore::default();
    let tree: JellyfishMerkleTree<'_, MockTreeStore, Sha256> = JellyfishMerkleTree::new(&db);

    let mut kvs: HashMap<KeyHash, OwnedValue> = HashMap::new();
    kvs.insert(KeyHash([1; 32]), b"abc".to_vec());
    kvs.insert(KeyHash([2; 32]), b"def".to_vec());
    kvs.insert(KeyHash([3; 32]), b"hij".to_vec());

    for (i, (key, value)) in kvs.clone().into_iter().enumerate() {
        let (_root_hash, write_batch) = tree
            .put_value_set(vec![(key, Some(value))], i as Version)
            .unwrap();
        // println!(">>> tree update batch : {:?}", write_batch);
        db.write_tree_update_batch(write_batch).unwrap();
    }

    let key = KeyHash([1; 32]);
    let key2 = KeyHash([2; 32]);
    let key3 = KeyHash([3; 32]);

    // let proof = tree.get_with_proof(key, 0);
    // println!(">>> proof 0 : {:?}", proof);
    // let proof = tree.get_with_proof(key, 1);
    // println!(">>> proof 1 : {:?}", proof);
    let proof1 = tree.get_with_proof(key, 2).unwrap();
    println!(">>> proof (key1) : {:?}\n\n", proof1);
    let proof2 = tree.get_with_proof(key2, 2).unwrap();
    println!(">>> proof (key2) : {:?}\n\n", proof2);
    let proof = tree.get_with_proof(key3, 2).unwrap();
    println!(">>> proof (key3) : {:?}\n\n", proof);

    // let multiproof = tree.get_multi_proof(&[key, key2], 2 as Version).unwrap();
    // println!(">>> multi proof : {:?}", multiproof);

    let tree_root = tree.get_root_hash(2).unwrap();

    let proof_component = proof1
        .1
        .verify(tree_root.clone(), key, Some(b"abc".to_vec()));
    println!("\n\n>> proof component : {:?}\n\n", proof_component);

    let proof_component = proof2
        .1
        .verify(tree_root.clone(), key2, Some(b"def".to_vec()));
    println!("\n\n>> proof component : {:?}\n\n", proof_component);

    println!(">>> tree root : {:?}", tree_root);

    // let proof_component = multiproof.verify(
    //     &[key, key2],
    //     tree_root,
    // );
    // println!(">> multi proof verification: {:?}", proof_component);
}
