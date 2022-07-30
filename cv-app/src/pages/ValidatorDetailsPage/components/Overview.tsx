import React from 'react';
import {Link, Table, TableBody, TableCell, TableRow, Typography,} from '@mui/material';
import {useRecoilValueLoadable} from 'recoil';
import {state} from './state';

function Overview(props: any) {
  const validator = useRecoilValueLoadable(state);

  return (
    <React.Fragment>
      {
        validator.state === 'hasValue' && validator.contents
          ? (<Table>
            <TableBody>
              <TableRow>
                <TableCell>
                  <Typography>Moniker</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{validator.contents.description.moniker}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Operator Address</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{validator.contents.operator_address}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Voting Power</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{parseInt(validator.contents.tokens).toLocaleString()}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Commission</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{`${(parseFloat(validator.contents.commission.commission_rates.rate) * 100).toFixed(2)}%`}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Details</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{validator.contents.description.details}</Typography>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Website</Typography>
                </TableCell>
                <TableCell>
                  <Link variant='mono' underline='none' noWrap={true} href={validator.contents.description.website}
                        target='_blank'
                        rel='noreferrer' sx={props.sx}>
                    {validator.contents.description.website}
                  </Link>
                </TableCell>
              </TableRow>
              <TableRow>
                <TableCell>
                  <Typography>Jailed</Typography>
                </TableCell>
                <TableCell>
                  <Typography>{validator.contents.jailed.toString()}</Typography>
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
