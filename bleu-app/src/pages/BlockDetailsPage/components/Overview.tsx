import React from 'react';
import {
  Box,
  IconButton,
  Link,
  Table,
  TableBody,
  TableCell,
  TableRow,
  Typography,
} from '@mui/material';
import { useRecoilValueLoadable } from 'recoil';
import { state } from './state';
import { BlockTxnLink, TransactionLink } from '../../../components/Link';
import { timeSince } from '../../../utils/time';
import { ArrowLeft, ArrowRight } from '@mui/icons-material';

function Overview(props: any) {
  const block = useRecoilValueLoadable(state);

  return (
    <React.Fragment>
      {
        block.state === 'hasValue' && block.contents
          ? (<Table>
            <TableBody>
              <TableRow>
                <TableCell>
                  <Typography>Block Height</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{block.contents.block_number}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Timestamp</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{timeSince(block.contents.block_timestamp)} ({new Date(+block.contents.block_timestamp * 1000).toLocaleString()})</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Transactions</Typography>
                </TableCell>
                <TableCell>
                  {block.contents.txn ? <BlockTxnLink blockNumber={block.contents.block_number} txn={block.contents.txn} /> : block.contents.txn}
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Difficulty</Typography>
                </TableCell>
                <TableCell>{block.contents.difficulty}</TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Size</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{block.contents.block_size}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Gas Used</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{block.contents.gas_used}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Gas Limit</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{block.contents.gas_limit}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Base Fee Per Gas</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{block.contents.base_fee_per_gas}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Hash</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{block.contents.hash}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Parent Hash</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{block.contents.parent_hash}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Nonce</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{block.contents.nonce}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell sx={{ borderBottom: 'none' }}>
                  <Typography>Extra data</Typography>
                </TableCell>
                <TableCell sx={{ borderBottom: 'none' }}>
                  <Typography>{Buffer.from(block.contents.extra_data, 'hex').toString()}</Typography>
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