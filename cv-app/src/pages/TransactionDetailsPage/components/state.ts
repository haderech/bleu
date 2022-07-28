import {atom} from 'recoil';

export const options = atom({
  key: 'TransactionPageOptions',
  default: {
    index: 0,
    txHash: '',
  },
});
