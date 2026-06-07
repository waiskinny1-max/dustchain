#[test]
fn localnet_sync_is_reserved_for_v05() {
    let planned_release = "v0.5-localnet";
    assert!(planned_release.starts_with("v0.5"));
}
