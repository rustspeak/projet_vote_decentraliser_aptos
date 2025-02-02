module Voting {

    resource struct Proposal {
        id: u64,
        description: vector<u8>,
        votes_for: u64,
        votes_against: u64,
    }

    public fun create_proposal(account: &signer, id: u64, description: vector<u8>) {
        let proposal = Proposal {
            id,
            description,
            votes_for: 0,
            votes_against: 0,
        };
        move_to(account, proposal);
    }

    public fun vote(account: &signer, proposal_id: u64, vote_for: bool) {
        let proposal = borrow_global_mut<Proposal>(@0x1, proposal_id);
        if (vote_for) {
            proposal.votes_for = proposal.votes_for + 1;
        } else {
            proposal.votes_against = proposal.votes_against + 1;
        }
    }
}

