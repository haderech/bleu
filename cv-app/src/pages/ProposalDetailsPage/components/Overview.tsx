import React from 'react';
import {Table, TableBody, TableCell, TableHead, TableRow, Typography,} from '@mui/material';
import {useRecoilValueLoadable} from 'recoil';
import {state} from './state';
import {getSimpleStatus, getSimpleType} from '../../../utils/message';

function Overview(props: any) {
  const proposal = useRecoilValueLoadable(state);

  return (
    <React.Fragment>
      {
        proposal.state === 'hasValue' && proposal.contents
          ? (<Table>
            <TableBody>
              <TableRow>
                <TableCell>
                  <Typography>Title</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{proposal.contents.content.title}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Total Deposit</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{proposal.contents.total_deposit.length > 0 ? `${proposal.contents.total_deposit[0]['amount']} ${proposal.contents.total_deposit[0]['denom']}` : '0 uatom'}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Voting Start</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{proposal.contents.voting_start_time}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Voting End</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{proposal.contents.voting_end_time}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Type</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{getSimpleType(proposal.contents.content['@type'])}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Submit Time</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{proposal.contents.submit_time}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Deposit End Time</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{proposal.contents.deposit_end_time}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Description</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{proposal.contents.content.description}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Status</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{getSimpleStatus(proposal.contents.status)}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell colSpan={2}>
                  <Table>
                    <TableHead>
                      <TableCell align='center'>Yes</TableCell>
                      <TableCell align='center'>Abstain</TableCell>
                      <TableCell align='center'>No</TableCell>
                      <TableCell align='center'>No with Veto</TableCell>
                    </TableHead>
                    <TableBody>
                      <TableCell
                        align='center'>{parseInt(proposal.contents.final_tally_result.yes).toLocaleString()}</TableCell>
                      <TableCell
                        align='center'>{parseInt(proposal.contents.final_tally_result.abstain).toLocaleString()}</TableCell>
                      <TableCell
                        align='center'>{parseInt(proposal.contents.final_tally_result.no).toLocaleString()}</TableCell>
                      <TableCell
                        align='center'>{parseInt(proposal.contents.final_tally_result.no_with_veto).toLocaleString()}</TableCell>
                    </TableBody>
                  </Table>
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
