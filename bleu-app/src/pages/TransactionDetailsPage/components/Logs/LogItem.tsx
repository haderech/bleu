import React from 'react';
import {
  Table,
  TableBody,
  TableCell,
  TableRow,
} from '@mui/material';
import { Log } from './state';
import { AddressLink } from '../../../../components/Link';

function LogItem(props: any) {
  const log = props.log as Log;

  return (
    <TableRow>
      <TableCell>
        <Table size='small'>
          <TableBody>
            <TableRow>
              <TableCell sx={{ border: 'none', textAlign: 'right' }}>
                Address
              </TableCell>
              <TableCell sx={{ border: 'none' }}>
                <AddressLink address={log.address} />
              </TableCell>
            </TableRow>
            {
              log.topics.map((row, index) => (
                <TableRow key={`topic-${index}`}>
                  <TableCell sx={{ border: 'none', textAlign: 'right' }}>
                    {index === 0 ? 'Topics' : ''}
                  </TableCell>
                  <TableCell sx={{ border: 'none' }}>
                    {`[${index}] ${row}`}
                  </TableCell>
                </TableRow>
              ))
            }
          </TableBody>
        </Table>
      </TableCell>
    </TableRow>
  );
}

export default LogItem;