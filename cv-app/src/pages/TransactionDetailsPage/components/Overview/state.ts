import {selector} from 'recoil';
import {api} from '../../../../utils/urlResolver';
import {options} from '../state';

export interface Transaction {
  code: number;
  data: string;
  fee: any[];
  gas_used: string;
  gas_wanted: string;
  height: string;
  memo: string;
  messages: any[];
  raw_log: string;
  timestamp: string;
  txhash: string;
}

export const state = selector<Transaction>({
  key: 'TransactionPageOverviewState',
  get: async ({get}) => {
    const opts = get(options);
    if (opts.txHash.length === 0) {
      return;
    }
    const res = await fetch(api('/tx/hash', opts.txHash));
    return await res.json();
  },
});

