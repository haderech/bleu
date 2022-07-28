import {atom, selector} from 'recoil';
import {api} from '../../../utils/urlResolver';

export interface State {
  optimism_tx_batches_id: number;
  batch_index: string;
  batch_timestamp: string;
  batch_size: string;
  l1_tx_hash: string;
  l1_block_number: string;
  batch_root: string;
  previous_total_elements: string;
  extra_data: string;
  submitter: string;
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
    if (!opts.isState) {
      const res = await fetch(api('/tx-batch/index', opts.blockNumber.toString()));
      return await res.json();
    } else {
      const res = await fetch(api('/stateroot-batch/index', opts.blockNumber.toString()));
      return await res.json();
    }
  },
});