import {atom} from 'recoil';

export interface Options {
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
    code: number;
    cosmos_tx_id: number;
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
  }[];
}

export const options = atom({
  key: 'TransactionsPageOptions',
  default: {
    numRows: 25,
    datetime: false,
  },
});

export const state = atom<State>({
  key: 'TransactionsPageState',
  default: {} as State,
});
