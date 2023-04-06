import { atom } from 'recoil';

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