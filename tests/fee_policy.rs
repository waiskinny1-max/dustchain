use dust_core::FeePolicy;

#[test]
fn normal_transfer_stays_at_minimum_fee() {
    let p = FeePolicy::default();
    assert_eq!(p.required_fee(184), 1);
}

#[test]
fn larger_transfer_gets_size_surcharge() {
    let p = FeePolicy::default();
    assert_eq!(p.required_fee(1025), 2);
}
