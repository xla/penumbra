use decaf377_fmd::{Clue, ExpandedClueKey};
use penumbra_crypto::Address;
use penumbra_proto::{transaction as pb, Protobuf};

use rand::{CryptoRng, RngCore};

#[derive(Clone, Debug)]
pub struct CluePlan {
    address: Address,
    precision_bits: usize,
    rseed: [u8; 32],
}

impl CluePlan {
    /// Create a new [`CluePlan`] associated with a given (possibly dummy) `Address`.
    pub fn new<R: CryptoRng + RngCore>(
        rng: &mut R,
        address: Address,
        precision_bits: usize,
    ) -> CluePlan {
        let mut rseed = [0u8; 32];
        rng.fill_bytes(&mut rseed);
        CluePlan {
            address,
            rseed,
            precision_bits,
        }
    }

    /// Create a [`Clue`] from the [`CluePlan`].
    pub fn clue(&self) -> Clue {
        let clue_key = self.address.clue_key();
        let expanded_clue_key = ExpandedClueKey::new(clue_key).expect("valid address");
        expanded_clue_key
            .create_clue_deterministic(self.precision_bits, self.rseed)
            .expect("can construct clue key")
    }
}

impl Protobuf<pb::CluePlan> for CluePlan {}

impl From<CluePlan> for pb::CluePlan {
    fn from(msg: CluePlan) -> Self {
        Self {
            address: Some(msg.address.into()),
            rseed: msg.rseed.to_vec().into(),
            precision_bits: (msg.precision_bits as u64).into(),
        }
    }
}

impl TryFrom<pb::CluePlan> for CluePlan {
    type Error = anyhow::Error;
    fn try_from(msg: pb::CluePlan) -> Result<Self, Self::Error> {
        Ok(Self {
            address: msg
                .address
                .ok_or_else(|| anyhow::anyhow!("missing address"))?
                .try_into()?,
            rseed: msg.rseed.as_ref().try_into()?,
            precision_bits: msg.precision_bits.try_into()?,
        })
    }
}