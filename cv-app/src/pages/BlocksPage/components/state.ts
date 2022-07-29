import {atom} from 'recoil';

export interface State {
  page_info: {
    count: number;
    page: number;
    total_count: number;
    total_page: number;
  },
  records: {
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
  }[];
}

export const options = atom({
  key: 'BlocksPageOptions',
  default: {
    numRows: 25,
    datetime: false,
  },
});

export const state = atom<State>({
  key: 'BlocksPageState',
  default: {} as State,
});
