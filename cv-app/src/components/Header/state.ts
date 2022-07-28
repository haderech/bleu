import {atom} from 'recoil';

interface HeaderOptions {
  index: number;
  anchorEl?: any;
  anchorCoords?: {
    top: number;
    right: number;
    bottom: number;
    left: number;
    width: number;
    height: number;
    x: number;
    y: number;
  } | null;
}

export const options = atom<HeaderOptions>({
  key: 'HeaderOptions',
  default: {
    index: -1,
    anchorEl: null,
    anchorCoords: null,
  },
});
