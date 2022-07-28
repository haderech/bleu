import React from 'react';
import {useRecoilValueLoadable} from 'recoil';
import {state} from './Logs/state';
import {
  Table,
  TableBody,
} from '@mui/material';
import LogItem from './Logs/LogItem';

function Logs() {
  const stateLoadable = useRecoilValueLoadable(state);
  return (
    <React.Fragment>
      {
        stateLoadable.state === 'hasValue' && stateLoadable.contents
        ? (
          <Table>
            <TableBody>
              {stateLoadable.contents.map((row, index) => (
                <LogItem key={`log-${index}`} log={row} />
              ))}
            </TableBody>
          </Table>
          )
        : null
      }
    </React.Fragment>
  );
}

export default Logs;
