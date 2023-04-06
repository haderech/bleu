import {selector} from 'recoil';
import {api} from '../../../../utils/urlResolver';
import {options} from '../state';

export interface Transaction {
  block_hash: string,
  block_number: string,
  chain_id: string,
  gas: string,
  gas_price: string,
  hash: string,
  max_fee_per_gas: string,
  nonce: string,
  public_key: string,
  tx_from: string,
  tx_input: string,
  tx_to: string,
  tx_type: string,
  tx_value: string,
  block_timestamp: string,
  contract_address: string,
  cumulative_gas_used: string,
  effective_gas_price: string,
  gas_used: string,
  status: string,
}

export const state = selector<Transaction>({
  key: 'TransactionPageOverviewState',
  get: async ({get}) => {
    const opts = get(options);
    if (opts.txHash.length === 0) {
      return;
    }
    const res = await fetch(api('/logs', opts.txHash));
    return await res.json();
  },
});

