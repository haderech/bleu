import {atom} from 'recoil';

export interface State {
  page_info: {
    count: number;
    page: number;
    total_count: number;
    total_page: number;
  },
  records: {
    batch_index: string;
    batch_root: string;
    batch_size: string;
    batch_timestamp: string;
    extra_data: string;
    l1_block_number: string;
    l1_tx_hash: string;
    optimism_tx_batches_id: number;
    previous_total_elements: string;
    submitter: string;
  }[];
}

export const options = atom({
  key: 'BlocksPageOptions',
  default: {
    numRows: 25,
    datetime: false,
  },
});

export const state = atom<State>({
  key: 'BlocksPageState',
  default: {} as State,
});