import {selector} from 'recoil';
import {api} from '../../../../utils/urlResolver';
import {options} from '../state';

export interface Log {
	address: string;
	topics: string[];
	log_data: string;
	block_number: string;
	transaction_hash: string;
	transaction_index: string;
	block_hash: string;
	log_index: string;
	removed: boolean;
}

export const state = selector<Log[]>({
  key: 'TransactionPageLogsState',
  get: async ({get}) => {
    const opts = get(options);
    if (opts.txHash.length === 0) {
      return;
    }
    if (opts.txHash.startsWith('0x')) {
      const res = await fetch(api('/logs', opts.txHash));
      return await res.json();
    } else {
    }
  },
});