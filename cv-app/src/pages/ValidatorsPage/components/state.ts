import {atom} from 'recoil';

export interface State {
  page_info: {
    count: number;
    page: number;
    total_count: number;
    total_page: number;
  },
  records: {
    commission: any;
    consensus_pubkey: any;
    delegator_shares: string;
    description: any;
    jailed: boolean;
    min_self_delegation: string;
    operator_address: string;
    status: string;
    tokens: string;
    unbonding_height: string;
    unbonding_time: string;
  }[];
}

export const options = atom({
  key: 'ValidatorsPageOptions',
  default: {
    numRows: 25,
    datetime: false,
  },
});

export const state = atom<State>({
  key: 'ValidatorsPageState',
  default: {} as State,
});
