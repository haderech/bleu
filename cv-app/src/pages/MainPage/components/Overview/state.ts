import {selector} from 'recoil';
import {api, nodeApi} from '../../../../utils/urlResolver';

interface Block {
  app_hash: string;
  consensus_hash: string;
  data_hash: string;
  evidence_hash: string;
  hash: string;
  height: string;
  last_block_id: string;
  last_commit_hash: number;
  last_results_hash: string;
  next_validators_hash: string;
  num_txs: number;
  proposer_address: string;
  time: string;
  validators_hash: string;
  version: string;
}

export const latestBlock = selector<Block>({
  key: 'LatestBlock',
  get: async () => {
    const res = await fetch(api('/block/height/latest'));
    return await res.json();
  },
});

interface Inflation {
  inflation: string;
}

export const inflation = selector<Inflation>({
  key: 'Inflation',
  get: async () => {
    const res = await fetch(api('/dashboard/inflation'));
    return await res.json();
  }
});

interface StakingPool {
  bonded_tokens : string;
  not_bonded_tokens : string;
}

export const stakingPool = selector<StakingPool>({
  key: 'StakingPool',
  get: async () => {
    const res = await fetch(api('/dashboard/staking/pool'));
    return await res.json();
  }
});

interface CommunityPool {
  denom : string;
  amount : string;
}

export const communityPool = selector<CommunityPool>({
  key: 'CommunityPool',
  get: async () => {
    const res = await fetch(api('/dashboard/community/pool'));
    return await res.json();
  }
});

interface NumOfVotingProposals {
  num_of_voting_proposals : number;
}

export const numOfVotingProposals = selector<NumOfVotingProposals>({
  key: 'NumOfVotingProposals',
  get: async () => {
    const res = await fetch(api('/dashboard/voting/proposal'));
    return await res.json();
  }
});
