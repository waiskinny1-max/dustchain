use dust_core::Hash;
use dust_node::{plan_from_status, p2p::process_request, PeerRequest, PeerStatus};
use dust_store::DustStore;

#[test]
fn peer_request_parser_accepts_status_and_block_fetch() {
    assert!(matches!(PeerRequest::parse("STATUS").unwrap(), PeerRequest::Status));
    assert!(matches!(PeerRequest::parse("GET_BLOCK 7").unwrap(), PeerRequest::GetBlock { height: 7 }));
}

#[test]
fn sync_plan_fetches_missing_remote_heights() {
    let status = PeerStatus { chain_id: "dust-local".to_string(), height: 4, tip_hash: Hash::ZERO, mempool_txs: 0 };
    let plan = plan_from_status("127.0.0.1:3030", 1, status);
    assert_eq!(plan.blocks_to_fetch, vec![2, 3, 4]);
    assert!(!plan.is_caught_up());
}

#[test]
fn status_request_reports_local_store() {
    let dir = tempfile::tempdir().unwrap();
    let store = DustStore::open(dir.path());
    store.init().unwrap();

    let response = process_request(PeerRequest::Status, &store).unwrap();
    assert!(response.starts_with("STATUS dust-local 0 "));
}
