use penumbra_crypto::stake::IdentityKey;

pub fn next_proposal_id() -> &'static str {
    "governance/next_proposal_id"
}

pub fn proposal_definition(proposal_id: u64) -> String {
    format!("governance/proposal/{}/data", proposal_id)
}

pub fn proposal_state(proposal_id: u64) -> String {
    format!("governance/proposal/{}/state", proposal_id)
}

pub fn proposal_deposit_amount(proposal_id: u64) -> String {
    format!("governance/proposal/{}/deposit_amount", proposal_id)
}

pub fn proposal_voting_start(proposal_id: u64) -> String {
    format!("governance/proposal/{}/voting_start", proposal_id)
}

pub fn proposal_voting_end(proposal_id: u64) -> String {
    format!("governance/proposal/{}/voting_end", proposal_id)
}

pub fn unfinished_proposals() -> &'static str {
    "governance/unfinished_proposals"
}

pub fn voting_validators_list(proposal_id: u64) -> String {
    format!("governance/proposal/{}/validator_vote/", proposal_id)
}

pub fn validator_vote(proposal_id: u64, identity_key: IdentityKey) -> String {
    format!(
        "governance/proposal/{}/validator_vote/{}",
        proposal_id, identity_key
    )
}
