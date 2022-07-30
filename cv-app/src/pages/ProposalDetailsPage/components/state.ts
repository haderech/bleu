import {atom, selector} from 'recoil';
import {api} from '../../../utils/urlResolver';

export interface State {
  content: any;
  deposit_end_time: string;
  final_tally_result: any;
  proposal_id: string;
  status: string;
  submit_time: string;
  total_deposit: any[];
  voting_end_time: string;
  voting_start_time: string;
}

export const options = atom({
  key: 'ProposalPageOptions',
  default: {
    index: 0,
    id: 0,
  },
});

export const state = selector<State>({
  key: 'ProposalPageState',
  get: async ({get}) => {
    const opts = get(options);
    if (opts.id === 0) {
      return;
    }
    const res = await fetch(api('/proposal/id', opts.id.toString()));
    return await res.json();
  },
});
