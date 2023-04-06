import {selector} from 'recoil';
import {api} from '../../../../utils/urlResolver';

interface Summary {
  latest_block_number: string;
  latest_block_gas_used: string;
  latest_block_gas_limit: string;
  total_transaction_count: number;
}

export const summary = selector<Summary>({
  key: 'MainPageSummary',
  get: async () => {
    const res = await fetch(api('/board/summary'));
    return await res.json();
  }
});