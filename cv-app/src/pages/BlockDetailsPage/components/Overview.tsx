import React from 'react';
import {Table, TableBody, TableCell, TableRow, Typography,} from '@mui/material';
import {useRecoilValueLoadable} from 'recoil';
import {state} from './state';
import {TxsLink} from '../../../components/Link';
import {timeSince} from "../../../utils/time";

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
                  <Typography>Height</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{block.contents.height}</Typography>
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
                  <Typography>Proposal Address</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{block.contents.proposer_address}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Num Txs</Typography>
                </TableCell>
                <TableCell>
                  <TxsLink height={block.contents.height} num_txs={block.contents.num_txs}></TxsLink>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Proposal Address</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{block.contents.proposer_address}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Time</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{`${timeSince(block.contents.time)} (${block.contents.time})`}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>App Hash</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{block.contents.app_hash}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Consensus Hash</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{block.contents.consensus_hash}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Data Hash</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{block.contents.data_hash}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Evidence Hash</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{block.contents.evidence_hash}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Last Block ID</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{block.contents.last_block_id}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Last Commit Hash</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{block.contents.last_commit_hash}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Last Results Hash</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{block.contents.last_results_hash}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Next Validator Hash</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{block.contents.next_validators_hash}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell sx={{borderBottom: 'none'}}>
                  <Typography>Validators Hash</Typography>
                </TableCell>
                <TableCell sx={{borderBottom: 'none'}}>
                  <Typography>{block.contents.validators_hash}</Typography>
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
