#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FeePolicy {
    pub base_fee: u64,
    pub fee_per_kb: u64,
    pub included_bytes: usize,
    pub max_priority_fee: u64,
    pub max_tx_size_bytes: usize,
    pub max_memo_bytes: usize,
    pub max_block_size_bytes: usize,
}

impl Default for FeePolicy {
    fn default() -> Self {
        Self {
            base_fee: 1,
            fee_per_kb: 1,
            included_bytes: 1024,
            max_priority_fee: 1_000,
            max_tx_size_bytes: 2_048,
            max_memo_bytes: 128,
            max_block_size_bytes: 1_048_576,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FeeBreakdown {
    pub encoded_size: usize,
    pub base_fee: u64,
    pub size_fee: u64,
    pub priority_fee: u64,
    pub required_fee: u64,
    pub paid_fee: u64,
    pub fee_per_byte_microunits: u64,
}

impl FeePolicy {
    pub fn required_fee(&self, encoded_size: usize) -> u64 {
        self.base_fee + self.size_fee(encoded_size)
    }

    pub fn size_fee(&self, encoded_size: usize) -> u64 {
        if encoded_size <= self.included_bytes {
            return 0;
        }
        let extra = encoded_size - self.included_bytes;
        let kb = (extra + 1023) / 1024;
        kb as u64 * self.fee_per_kb
    }

    pub fn breakdown(&self, encoded_size: usize, priority_fee: u64) -> FeeBreakdown {
        let size_fee = self.size_fee(encoded_size);
        let required_fee = self.base_fee + size_fee;
        let paid_fee = required_fee.saturating_add(priority_fee);
        let fee_per_byte_microunits = if encoded_size == 0 {
            0
        } else {
            paid_fee.saturating_mul(1_000_000) / encoded_size as u64
        };
        FeeBreakdown {
            encoded_size,
            base_fee: self.base_fee,
            size_fee,
            priority_fee,
            required_fee,
            paid_fee,
            fee_per_byte_microunits,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_kb_is_covered_by_base_fee() {
        let p = FeePolicy::default();
        assert_eq!(p.required_fee(180), 1);
        assert_eq!(p.required_fee(1024), 1);
        assert_eq!(p.required_fee(1025), 2);
    }
}
