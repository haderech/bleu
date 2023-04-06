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

export function api(path: string, key?: string, params?: string | object) {
  return consts.BleuServerEndpoint + '/api/v1/ethereum' + path + (key ? '/' + key : '') + paramsToString(params);
}
