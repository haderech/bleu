import {selector} from 'recoil';
import {api} from '../../../../utils/urlResolver';
import {options} from '../state';

export interface Transaction {
  l1_origin_tx_hash: string;
  tx_ext: {
    contract_address: string;
    gas_used: string;
    l1_state_batch_index: string;
    l1_state_root_submission_tx_hash: string;
    l1_submission_tx_hash: string;
    l1_tx_batch_index: string;
    tx: {
      block_hash: string;
      block_number: string;
      from_address: string;
      gas: string;
      gas_price: string;
      hash: string;
      index: string;
      l1_block_number: string;
      l1_timestamp: string;
      l1_tx_origin: string;
      nonce: string;
      optimism_block_txs_id: number;
      queue_index: string;
      queue_origin: string;
      raw_tx: string;
      to_address: string;
      tx_index: string;
      tx_input: string;
      tx_type: string;
      value: string;
    };
    state: string;
  }
}

export const state = selector<Transaction>({
  key: 'TransactionPageOverviewState',
  get: async ({get}) => {
    const opts = get(options);
    if (opts.txHash.length === 0) {
      return;
    }
    if (opts.txHash.startsWith('0x')) {
      const res = await fetch(api('/tx/hash', opts.txHash));
      return await res.json();
    } else {
      const res = await fetch(api('/tx/index', opts.txHash));
      return await res.json();
    }
  },
});

