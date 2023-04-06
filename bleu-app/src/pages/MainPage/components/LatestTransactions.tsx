import React from 'react';
import InfoCard from '../../../components/InfoCard';
import { timeSince } from '../../../utils/time';
import {
  Avatar,
  Box,
  Link,
  Table,
  TableBody,
  TableCell,
  TableRow,
  Tooltip,
  Typography,
} from '@mui/material';
import { Loadable, selector, useRecoilValueLoadable } from 'recoil';
import { api } from '../../../utils/urlResolver';
import { AddressLink, TransactionLink } from '../../../components/Link';

interface Transaction {
  hash: string;
  tx_from: string;
  tx_to: string;
  contract_address: string;
  tx_value: string;
  block_timestamp: string;
};

const latestTransactionsState = selector({
  key: 'LatestTransactions',
  get: async () => {
    const res = await fetch(api('/txs/latest'));
    const txs = await res.json();
    return txs;
  },
});

const tableRow: Readonly<any> = {
  py: '10px',
  '&: nth-of-type(1)': {
    pt: 0,
  },
};

const tableCell: Readonly<any> = {
  px: '6px',
  py: 'inherit',
};

const tableCellLast: Readonly<any> = {
  ...tableCell,
  borderBottom: 'none',
  pb: '0px',
};

const content: Readonly<any> = {
  display: 'flex',
  flexDirection: 'column',
};

function LatestTransactions() {
  const latestTransactions: Loadable<Transaction[]> = useRecoilValueLoadable(latestTransactionsState);
  return (
    <InfoCard title='Latest Transactions' buttonProps={{ label: 'View all transactions', href: '/txs' }} sx={{ height: '500px' }}>
      <Table>
        <TableBody>
          {
            latestTransactions.state === 'hasValue'
              ? latestTransactions.contents.map((row, index) => (
                <TableRow key={index} sx={tableRow}>
                  <TableCell sx={(index === latestTransactions.contents.length - 1) ? tableCellLast : tableCell}>
                    <Box sx={{ display: 'flex', alignItems: 'center', gap: '12px', width: '100%' }}>
                      <Avatar>Tx</Avatar>
                      <Box sx={{ flexGrow: 1, flexBasis: 0, width: 0 }}>
                        <Box sx={{ display: 'flex', flexGrow: 1, gap: '8px' }}>
                          <Typography>Tx</Typography>
                          <TransactionLink sx={{ width: 0, flexGrow: 1, flexBasis: 0 }} hash={row.hash} />
                        </Box>
                        <Box sx={{ display: 'flex' }}>
                          <Box sx={{ display: 'flex', flexGrow: 1, gap: '8px' }}>
                            <Typography>From</Typography>
                            <AddressLink sx={{ width: 0, flexGrow: 1, flexBasis: 0 }} address={row.tx_from} />
                          </Box>
                          <Box sx={{ display: 'flex', flexGrow: 1, gap: '8px' }}>
                            <Typography>To</Typography>
                            {
                              row.tx_to
                                ? <AddressLink sx={{ width: 0, flexGrow: 1, flexBasis: 0 }} address={row.tx_to} />
                                : <Tooltip title={row.contract_address}>
                                  <Link underline='none' href={`/account/${row.contract_address}`}>
                                    Contract Creation
                                  </Link>
                                </Tooltip>
                            }
                          </Box>
                        </Box>
                        <Box sx={{ display: 'flex', gap: '10px' }}>
                          <Typography variant='body2'>{+row.tx_value / Math.pow(10, 18)}</Typography>
                          <Typography variant='body2' color='text.secondary'>{timeSince(row.block_timestamp)}</Typography>
                        </Box>
                      </Box>
                    </Box>
                  </TableCell>
                </TableRow>
              ))
              : null
          }
        </TableBody>
      </Table>
    </InfoCard>
  );
}

export default LatestTransactions;