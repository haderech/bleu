import {atom} from 'recoil';

export interface State {
  page_info: {
    count: number;
    page: number;
    total_count: number;
    total_page: number;
  },
  records: {
    content: any;
    deposit_end_time: string;
    final_tally_result: any;
    proposal_id: string;
    status: string;
    submit_time: string;
    total_deposit: any[];
    voting_end_time: string;
    voting_start_time: string;
  }[];
}

export const options = atom({
  key: 'ProposalsPageOptions',
  default: {
    numRows: 25,
    datetime: false,
  },
});

export const state = atom<State>({
  key: 'ProposalsPageState',
  default: {} as State,
});
