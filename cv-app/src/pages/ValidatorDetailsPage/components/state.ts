import {atom, selector} from 'recoil';
import {api} from '../../../utils/urlResolver';

export interface State {
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
}

export const options = atom({
  key: 'ValidatorPageOptions',
  default: {
    index: 0,
    address: '',
  },
});

export const state = selector<State>({
  key: 'ValidatorPageState',
  get: async ({get}) => {
    const opts = get(options);
    if (opts.address === '') {
      return;
    }
    const res = await fetch(api('/validator/address', opts.address));
    return await res.json();
  },
});
