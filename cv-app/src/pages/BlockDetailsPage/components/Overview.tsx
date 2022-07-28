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
import {useRecoilValueLoadable} from 'recoil';
import {state} from './state';
import {L1TransactionLink} from '../../../components/Link';
import {timeSince} from '../../../utils/time';
import {ArrowLeft, ArrowRight} from '@mui/icons-material';

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
                  <Typography>Batch Index</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{block.contents.batch_index}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>L1 Timestamp</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{timeSince(block.contents.batch_timestamp)} ({new Date(+block.contents.batch_timestamp * 1000).toLocaleString()})</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Batch Size</Typography>
                </TableCell>
                <TableCell>
                  <Link underline='none' href={`/txs?blockNum=${block.contents.batch_index}&isState=${props.isState}`}>{block.contents.batch_size}</Link>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>L1 Transaction Hash</Typography>
                </TableCell>
                <TableCell>
                  <L1TransactionLink hash={block.contents.l1_tx_hash} />
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>L1 Block Number</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{block.contents.l1_block_number}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Batch root</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{block.contents.batch_root}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Previous Total Elements</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{block.contents.previous_total_elements}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell sx={{borderBottom:'none'}}>
                  <Typography>Extra data</Typography>
                </TableCell>
                <TableCell sx={{borderBottom:'none'}}>
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