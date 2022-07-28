import {atom, selector} from 'recoil';

export const options = atom({
  key: 'AccountPageTransactionsOptions',
  default: {
    address: '',
    numRows: 25,
    datetime: false,
  },
});

interface State {
  page_info: {
    count: number;
    page: number;
    total_count: number;
    total_page: number;
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

export const state = atom<State>({
  key: 'AccountPageTransactionsState',
  default: {} as State,
});