use borsh::{self, BorshSerialize};

/// Instruction.
pub struct LifinityTokenSwap {
    pub _in_amount: Option<u64>,
    pub _minimum_out_amount: u64,
    pub _platform_fee_bps: u8,
}
impl borsh::ser::BorshSerialize for LifinityTokenSwap
where
    Option<u64>: borsh::ser::BorshSerialize,
    u64: borsh::ser::BorshSerialize,
    u8: borsh::ser::BorshSerialize,
{
    fn serialize<W: borsh::maybestd::io::Write>(
        &self,
        writer: &mut W,
    ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
        borsh::BorshSerialize::serialize(&self._in_amount, writer)?;
        borsh::BorshSerialize::serialize(&self._minimum_out_amount, writer)?;
        borsh::BorshSerialize::serialize(&self._platform_fee_bps, writer)?;
        Ok(())
    }
}
impl borsh::de::BorshDeserialize for LifinityTokenSwap
where
    Option<u64>: borsh::BorshDeserialize,
    u64: borsh::BorshDeserialize,
    u8: borsh::BorshDeserialize,
{
    fn deserialize(buf: &mut &[u8]) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
        Ok(Self {
            _in_amount: borsh::BorshDeserialize::deserialize(buf)?,
            _minimum_out_amount: borsh::BorshDeserialize::deserialize(buf)?,
            _platform_fee_bps: borsh::BorshDeserialize::deserialize(buf)?,
        })
    }
}
impl anchor_lang::InstructionData for LifinityTokenSwap {
    fn data(&self) -> Vec<u8> {
        let mut d = [0, 49, 246, 1, 36, 153, 11, 93].to_vec();
        d.append(&mut self.try_to_vec().expect("Should always serialize"));
        d
    }
}
