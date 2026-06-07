use dust_core::{Address, Chain, FeePolicy};

#[test]
fn empty_local_chain_has_zero_height() {
    let chain = Chain::new("dust-local", FeePolicy::default());
    assert_eq!(chain.height(), 0);
    assert_eq!(chain.tip().unwrap().header.producer, Address::ZERO);
}
