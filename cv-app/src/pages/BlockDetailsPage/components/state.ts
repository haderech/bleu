import {atom, selector} from 'recoil';
import {api} from '../../../utils/urlResolver';

export interface State {
  app_hash: string;
  chain_id: string;
  consensus_hash: string;
  data_hash: string;
  evidence_hash: string;
  hash: string;
  height: string;
  last_block_id: string;
  last_commit_hash: string;
  last_results_hash: string;
  next_validators_hash: string;
  num_txs: number;
  proposer_address: string;
  time: string;
  validators_hash: string;
  version: string;
}

export const options = atom({
  key: 'BlockPageOptions',
  default: {
    index: 0,
    height: 0,
  },
});

export const state = selector<State>({
  key: 'BlockPageState',
  get: async ({get}) => {
    const opts = get(options);
    if (opts.height === 0) {
      return;
    }
    const res = await fetch(api('/block/height', opts.height.toString()));
    return await res.json();
  },
});
