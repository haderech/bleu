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
    contract_address: string;
    gas_used: string;
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
    }
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