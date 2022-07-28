import {selector} from 'recoil';
import {ethers} from 'ethers';
import {L2JsonRpcEndpoint} from '../../../../utils/consts';
import {api} from '../../../../utils/urlResolver';
import erc20ABI from './erc20.abi.json';

const provider = new ethers.providers.JsonRpcProvider(L2JsonRpcEndpoint);
const erc20 = new ethers.Contract('0xDeadDeAddeAddEAddeadDEaDDEAdDeaDDeAD0000', erc20ABI, provider);

export const wrappedEth = selector({
  key: 'MainPageL2WrappedEther',
  get: async () => {
    return await erc20.totalSupply();
  },
});

interface Summary {
  latest_state_batch_index: string;
  latest_tx_batch_index: string;
  tx_count: number;
}

export const summary = selector<Summary>({
  key: 'MainPageSummary',
  get: async () => {
    const res = await fetch(api('/board/summary'));
    return await res.json();
  }
});