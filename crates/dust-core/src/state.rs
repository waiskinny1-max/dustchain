use std::collections::BTreeMap;

use crate::{Account, Address, DustError, FeePolicy, Hash, Result, SignedTransaction};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct State {
    accounts: BTreeMap<Address, Account>,
}

impl State {
    pub fn new() -> Self {
        Self { accounts: BTreeMap::new() }
    }

    pub fn accounts(&self) -> impl Iterator<Item = &Account> {
        self.accounts.values()
    }

    pub fn ensure_account(&mut self, address: Address) -> &mut Account {
        self.accounts.entry(address).or_insert_with(|| Account::new(address))
    }

    pub fn get(&self, address: &Address) -> Account {
        self.accounts.get(address).cloned().unwrap_or_else(|| Account::new(*address))
    }

    pub fn set_account(&mut self, account: Account) {
        self.accounts.insert(account.address, account);
    }

    pub fn credit(&mut self, address: Address, amount: u64) {
        let account = self.ensure_account(address);
        account.balance = account.balance.saturating_add(amount);
    }

    pub fn root_hash(&self) -> Hash {
        let mut bytes = Vec::new();
        for account in self.accounts.values() {
            bytes.extend_from_slice(account.address.as_bytes());
            bytes.extend_from_slice(&account.balance.to_le_bytes());
            bytes.extend_from_slice(&account.nonce.to_le_bytes());
        }
        Hash::digest(bytes)
    }

    pub fn apply_transaction<F>(&mut self, signed: &SignedTransaction, policy: &FeePolicy, verifier: F) -> Result<u64>
    where
        F: Fn(&SignedTransaction) -> bool,
    {
        validate_transaction_against_state(self, signed, policy, verifier)?;
        let paid_fee = signed.paid_fee(policy);
        let total_debit = signed.tx.amount.saturating_add(paid_fee);

        let mut sender = self.get(&signed.tx.from);
        sender.balance -= total_debit;
        sender.nonce += 1;
        self.set_account(sender);

        let mut recipient = self.get(&signed.tx.to);
        recipient.balance = recipient.balance.saturating_add(signed.tx.amount);
        self.set_account(recipient);

        Ok(paid_fee)
    }
}

pub fn validate_transaction_against_state<F>(state: &State, signed: &SignedTransaction, policy: &FeePolicy, verifier: F) -> Result<()>
where
    F: Fn(&SignedTransaction) -> bool,
{
    if signed.tx.amount == 0 {
        return Err(DustError::ZeroAmount);
    }
    if signed.tx.memo.len() > policy.max_memo_bytes {
        return Err(DustError::MemoTooLarge { max: policy.max_memo_bytes, received: signed.tx.memo.len() });
    }
    if signed.encoded_size > policy.max_tx_size_bytes {
        return Err(DustError::TransactionTooLarge { max: policy.max_tx_size_bytes, received: signed.encoded_size });
    }
    if signed.tx.priority_fee > policy.max_priority_fee {
        return Err(DustError::PriorityFeeTooHigh { max: policy.max_priority_fee, received: signed.tx.priority_fee });
    }

    let required_fee = policy.required_fee(signed.encoded_size);
    if signed.tx.max_fee < required_fee {
        return Err(DustError::InsufficientMaxFee { required: required_fee, received: signed.tx.max_fee });
    }

    let sender = state.get(&signed.tx.from);
    if sender.nonce != signed.tx.nonce {
        return Err(DustError::NonceMismatch { address: signed.tx.from, expected: sender.nonce, received: signed.tx.nonce });
    }

    let required_balance = signed.tx.amount.saturating_add(signed.paid_fee(policy));
    if sender.balance < required_balance {
        return Err(DustError::InsufficientBalance { address: signed.tx.from, required: required_balance, available: sender.balance });
    }

    if !verifier(signed) {
        return Err(DustError::BadSignature);
    }

    Ok(())
}
