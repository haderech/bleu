import React from 'react';
import {
  Box,
  Table,
  TableBody,
  TableCell,
  TableRow,
  TextareaAutosize,
  Typography,
} from '@mui/material';
import { useRecoilValueLoadable } from 'recoil';
import { state } from './Overview/state';
import { AddressLink } from '../../../components/Link';
import { timeSince } from '../../../utils/time';
import { toEther, txFee } from '../../../utils/ethUtils';

function parseInputData(input: string): string {
  if (input.length < 10) {
    return input;
  }
  let ret = `MethodID: ${input.slice(0, 10)}`;
  let i = 0;
  input = input.slice(10);
  while (input.length > 0) {
    ret += `\n[${i}]` + (i < 10 ? ' ' : '') + ` ${input.slice(0, 64)}`;
    input = input.slice(64);
    i += 1;
  }
  return ret;
}

function Overview() {
  const stateLoadable = useRecoilValueLoadable(state);

  return (
    <React.Fragment>
      {
        stateLoadable.state === 'hasValue' && stateLoadable.contents
          ? (<Table>
            <TableBody>
              <TableRow>
                <TableCell>
                  <Typography>Transaction Hash</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{stateLoadable.contents.hash}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Status</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{+stateLoadable.contents.status ? 'Success' : 'Failed'}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Block</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{stateLoadable.contents.block_number}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Timestamp</Typography>
                </TableCell>
                <TableCell>
                  <Box sx={{ display: 'flex', gap: '8px' }}>
                    <Typography>{timeSince(stateLoadable.contents.block_timestamp)}</Typography>
                    <Typography>({new Date(+stateLoadable.contents.block_timestamp).toLocaleString()})</Typography>
                  </Box>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>From</Typography>
                </TableCell>
                <TableCell>
                  <AddressLink address={stateLoadable.contents.tx_from} />
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>To</Typography>
                </TableCell>
                <TableCell>
                  {
                    stateLoadable.contents.tx_to
                      ? <AddressLink address={stateLoadable.contents.tx_to} />
                      : (<React.Fragment>
                        [Contract <AddressLink address={stateLoadable.contents.contract_address} /> Created]
                      </React.Fragment>)
                  }
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Value</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{toEther(stateLoadable.contents.tx_value)} Unit</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Transaction Fee</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{txFee(stateLoadable.contents.gas_used, stateLoadable.contents.gas_price)} Unit</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Gas Price</Typography>
                </TableCell>
                <TableCell>
                  <Typography>
                    {(+stateLoadable.contents.gas_price).toLocaleString()} Wei
                  </Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Gas Used</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{(+stateLoadable.contents.gas_used).toLocaleString()}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Nonce</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{stateLoadable.contents.nonce}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell sx={{ borderBottom: 'none' }}>
                  <Typography>Input Data</Typography>
                </TableCell>
                <TableCell sx={{ borderBottom: 'none' }}>
                  <TextareaAutosize style={{ width: '100%', resize: 'vertical', backgroundColor: '#f5f5f5', color: '#74838e' }}
                    aria-label='transaction-details-input-data'
                    readOnly
                    defaultValue={stateLoadable.contents.tx_to ? parseInputData(stateLoadable.contents.tx_input) : stateLoadable.contents.tx_input}
                  />
                </TableCell>
              </TableRow>
            </TableBody>
          </Table>)
          : null
      }
    </React.Fragment>
  );
}

export default Overview;