import {atom, selector} from 'recoil';
import {api} from '../../../utils/urlResolver';
import {L2JsonRpcEndpoint} from '../../../utils/consts';

export const options = atom({
  key: 'AccountPageOptions',
  default: {
    address: '',
  },
});

export const tabIndex = atom({
  key: 'AccountPageTabIndex',
  default: 0,
});

export const balance = selector({
  key: 'AccountPageState',
  get: async ({get}) => {
    const opts = get(options);
    try {
      if (opts.address.length > 0) {
        const res = await fetch(L2JsonRpcEndpoint, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify({
            jsonrpc: '2.0',
            method: 'eth_getBalance',
            id: 0,
            params: [opts.address, 'latest'],
          }),
        });
        const json = await res.json();
        return BigInt(json.result).toString();
      }
      return '';
    } catch (e) {
      return '';
    }
  },
});