import * as consts from './consts';

function paramsToString(params?: string | object): string {
  if (params) {
    if (typeof params === 'object') {
      params = Object.entries(params).reduce((prev, curr) => {
        return prev + (prev.length > 0 ? '&' : '') + curr[0] + '=' + curr[1];
      }, '');
    }
    params = '?' + params;
  }
  return params || '';
}

export function l1Explorer(path: string, key?: string, params?: string | object) {
  return consts.L1ExplorerEndpoint + path + (key ? '/' + key : '') + paramsToString(params);
}

export function api(path: string, key?: string, params?: string | object) {
  return consts.CvServerEndpoint + '/api/v1/cosmos' + path + (key ? '/' + key : '') + paramsToString(params);
}
