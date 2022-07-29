import React from 'react';
import {Table, TableBody, TableCell, TableHead, TableRow, Typography,} from '@mui/material';
import {useRecoilValueLoadable} from 'recoil';
import {state} from './Overview/state';
import {BlockLink} from '../../../components/Link';
import ReactJson from 'react-json-view';
import {timeSince} from '../../../utils/time';

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
                  <Typography>Tx Hash</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{stateLoadable.contents.txhash}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Height</Typography>
                </TableCell>
                <TableCell>
                  <BlockLink height={stateLoadable.contents.height}/>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Result</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{stateLoadable.contents.code == 0 ? 'success' : 'fail'}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Fee</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{stateLoadable.contents.fee.length > 0 ? `${stateLoadable.contents.fee[0]['amount']}${stateLoadable.contents.fee[0]['denom']}` : '0uatom'}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Gas Used</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{stateLoadable.contents.gas_used}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Gas Used</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{stateLoadable.contents.gas_wanted}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Memo</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{stateLoadable.contents.memo}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Timestamp</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{`${timeSince(stateLoadable.contents.timestamp)} (${stateLoadable.contents.timestamp})`}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Messages</Typography>
                </TableCell>
                <TableCell>
                  {stateLoadable.contents.messages.length > 0 ? stateLoadable.contents.messages.map((message: any, index: number) => {
                    return (<Table>
                      <TableHead>
                        <TableRow>
                          <TableCell colSpan={2}>
                            Message#{index + 1}
                          </TableCell>
                        </TableRow>
                      </TableHead>
                      <TableBody>{
                        Object.entries(message).map((kv: any) => {
                          return (
                            <TableRow>
                              <TableCell>
                                {kv[0].startsWith('@') ? kv[0].slice(1) : kv[0]}
                              </TableCell>
                              <TableCell>
                                {(typeof kv[1] == 'object') ? <ReactJson src={kv[1]} collapsed/> : kv[1]}
                              </TableCell>
                            </TableRow>);
                        })
                      }</TableBody></Table>);
                  }) : null}
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Raw Log</Typography>
                </TableCell>
                <TableCell>
                  <ReactJson src={JSON.parse(stateLoadable.contents.raw_log)} collapsed/>
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
