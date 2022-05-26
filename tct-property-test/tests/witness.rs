#[macro_use]
extern crate proptest_derive;

use std::collections::HashSet;

use proptest::{arbitrary::*, prelude::*};

use penumbra_tct::{Commitment, CommitmentStrategy, Tree, Witness};

const MAX_USED_COMMITMENTS: usize = 3;
const MAX_TIER_ACTIONS: usize = 10;

#[derive(Debug, Copy, Clone, Arbitrary)]
#[proptest(params("Vec<Commitment>"))]
enum Action {
    Insert(
        Witness,
        #[proptest(strategy = "CommitmentStrategy::one_of(params.clone())")] Commitment,
    ),
    EndBlock,
    EndEpoch,
    Forget(#[proptest(strategy = "CommitmentStrategy::one_of(params)")] Commitment),
}

impl Action {
    fn apply(&self, tree: &mut Tree) -> anyhow::Result<()> {
        // Predict the position of the next insertion
        let predicted_position = tree.position();

        match self {
            Action::Insert(witness, commitment) => {
                // Insert the commitmentg
                tree.insert(*witness, *commitment)?;

                // If the insertion succeeded, the position must have been non-`None`
                assert!(predicted_position.is_some());

                // If the commitment was witnessed, the position must match the position of the
                // commitment when retrieved, and the proof must validate and contain the correct
                // commitment
                if matches!(witness, Witness::Keep) {
                    let commitment_position = tree.position_of(*commitment);
                    assert!(commitment_position.is_some());
                    assert_eq!(predicted_position, commitment_position);

                    let proof = tree.witness(*commitment).unwrap();
                    assert_eq!(*commitment, proof.commitment());

                    assert!(proof.verify(tree.root()).is_ok());
                }

                // Check that the position advanced by one commitment
                let old_position = predicted_position.unwrap();
                let new_position = tree.position().unwrap();

                assert_eq!(new_position.epoch(), old_position.epoch());
                assert_eq!(new_position.block(), old_position.block());
                assert_eq!(new_position.commitment(), old_position.commitment() + 1);
            }
            Action::EndBlock => {
                tree.end_block()?;

                let old_position = predicted_position.unwrap();
                let new_position = tree.position().unwrap();

                assert_eq!(new_position.epoch(), old_position.epoch());
                assert_eq!(new_position.block(), old_position.block() + 1);
                assert_eq!(new_position.commitment(), 0);
            }
            Action::EndEpoch => {
                tree.end_epoch()?;

                let old_position = predicted_position.unwrap();
                let new_position = tree.position().unwrap();

                assert_eq!(new_position.epoch(), old_position.epoch() + 1);
                assert_eq!(new_position.block(), 0);
                assert_eq!(new_position.commitment(), 0);
            }
            Action::Forget(commitment) => {
                let exists = tree.witness(*commitment).is_some();
                let result = tree.forget(*commitment);
                assert_eq!(exists, result);
            }
        };

        Ok(())
    }
}

proptest! {
    #[test]
    fn index_correct(
        actions in
            prop::collection::vec(any::<Commitment>(), 1..MAX_USED_COMMITMENTS)
                .prop_flat_map(|commitments| {
                    prop::collection::vec(any_with::<Action>(commitments), 1..MAX_TIER_ACTIONS)
                })
    ) {
        let mut tree = Tree::new();

        let mut commitments_added = HashSet::new();

        for action in &actions {
            match action {
                Action::Insert (Witness::Keep, commitment) => {
                    commitments_added.insert(commitment);
                },
                Action::Forget (commitment) => {
                    commitments_added.remove(&commitment);
                },
                _ => {}
            }
            action.apply(&mut tree).unwrap();
        }

        // Check generated commitments
        for commitment in commitments_added {
            let commitment_position = tree.position_of(*commitment);
            assert!(commitment_position.is_some());

            let proof = tree.witness(*commitment).unwrap();
            assert_eq!(*commitment, proof.commitment());

            assert!(proof.verify(tree.root()).is_ok());
        }
    }
}