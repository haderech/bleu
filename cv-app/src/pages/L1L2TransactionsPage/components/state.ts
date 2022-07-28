import {atom} from 'recoil';

export interface Options {
  page: number;
  numRows: number;
  datetime: boolean;
}

export interface State {
  page_info: {
    page: number;
    count: number;
    total_page: number;
    total_count: number;
  };
  records: {
    gas_limit: string;
    l1_block_number: string;
    l1_tx_hash: string;
    l1_tx_origin: string;
    l2_tx_hash: string;
    queue_index: string;
    timestamp: string;
  }[];
}

export const options = atom({
  key: 'L1L2TransactionsPageOptions',
  default: {
    numRows: 25,
    datetime: false,
  },
});

export const state = atom<State>({
  key: 'L1L2TransactionsPageState',
  default: {} as State,
});