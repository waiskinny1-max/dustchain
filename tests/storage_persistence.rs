use dust_store::DustStore;

#[test]
fn init_creates_metadata_and_stats() {
    let dir = tempfile::tempdir().unwrap();
    let store = DustStore::open(dir.path().join("chain"));
    store.init().unwrap();

    let stats = store.db_stats().unwrap();
    assert_eq!(stats.height, 0);
    assert_eq!(stats.block_files, 0);
    assert_eq!(stats.mempool_files, 0);
    assert!(stats.metadata_bytes > 0);
}

#[test]
fn wallet_survives_store_reopen() {
    let dir = tempfile::tempdir().unwrap();
    let data_dir = dir.path().join("chain");
    let store = DustStore::open(&data_dir);
    store.init().unwrap();
    let wallet = store.create_wallet("alice").unwrap();

    let reopened = DustStore::open(&data_dir);
    let loaded = reopened.wallet("alice").unwrap();
    assert_eq!(wallet.address, loaded.address);
}
