import {atom, selector} from 'recoil';
import {api} from '../../../utils/urlResolver';

export interface State {
  ethereum_blocks_id: number;
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
}

export const options = atom({
  key: 'BlockPageOptions',
  default: {
    index: 0,
    blockNumber: 0,
    isState: false,
  },
});

export const state = selector<State>({
  key: 'BlockPageState',
  get: async ({get}) => {
    const opts = get(options);
    if (opts.blockNumber === 0) {
      return;
    }
    const res = await fetch(api('/block', opts.blockNumber.toString()));
    return await res.json();
  },
});