import { atom } from 'recoil';

export interface State {
  page_info: {
    count: number;
    page: number;
    total_count: number;
    total_page: number;
  },
  records: {
    optimism_tx_batches_id: number;
    author: string;
    base_fee_per_gas: string;
    block_number: string;
    block_size: string;
    block_timestamp: string;
    difficulty: string;
    extra_data: string;
    gas_limit: string;
    gas_used: string;
    hash: string;
    logs_bloom: string;
    miner: string;
    nonce: string;
    parent_hash: string;
    receipt_root: string;
    sha3_uncles: string;
    state_root: string;
    total_difficulty: string;
    transaction_root: string;
    uncles: string;
    txn: number;
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