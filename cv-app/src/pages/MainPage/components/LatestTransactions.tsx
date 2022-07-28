import React, {useEffect} from 'react';
import InfoCard from '../../../components/InfoCard';
import {timeSince} from '../../../utils/time';
import {
  Avatar,
  Box,
  Table,
  TableBody,
  TableCell,
  TableRow,
  Typography,
} from '@mui/material';
import {atom, useRecoilState} from 'recoil';
import {api} from '../../../utils/urlResolver';
import {L2AddressLink, L2TransactionLink} from '../../../components/Link';
import {MainPageAutoRefresh} from '../../../utils/consts';

interface Transaction {
  tx_hash: string;
  from_address: string;
  to_address: string;
  value: string;
  tx_timestamp: string;
};

const latestTransactionsState = atom<Transaction[]>({
  key: 'LatestTransactions',
  default: [],
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

function LatestTransactions() {
  const [state, setState] = useRecoilState(latestTransactionsState);

  const reload = () => {
    (async () => {
      const res = await fetch(api('/tx/latest'));
      const json = await res.json();
      setState(json);
    })();
  };

  useEffect(() => {
    reload();
    if (MainPageAutoRefresh) {
      const id = setInterval(reload, 10000);
      return () => {
        clearInterval(id);
      };
    }
  }, []);

  return (
    <InfoCard title='Latest Transactions' buttonProps={{label:'View all transactions',href:'/txs'}} sx={{height:'500px'}}>
      <Table>
        <TableBody>
          {
            state.map((row, index) => (
              <TableRow key={index} sx={tableRow}>
                <TableCell sx={(index === state.length - 1) ? tableCellLast : tableCell}>
                  <Box sx={{display:'flex',alignItems:'center',gap:'12px',width:'100%'}}>
                    <Avatar>Tx</Avatar>
                    <Box sx={{flexGrow:1,flexBasis:0,width:0}}>
                      <Box sx={{display:'flex',flexGrow:1,gap:'8px'}}>
                        <Typography>Tx#</Typography>
                        <L2TransactionLink sx={{width:0,flexGrow:1,flexBasis:0}} hash={row.tx_hash} />
                      </Box>
                      <Box sx={{display:'flex'}}>
                        <Box sx={{display:'flex',flexGrow:1, gap:'8px'}}>
                          <Typography>From</Typography>
                          <L2AddressLink sx={{width:0,flexGrow:1,flexBasis:0}} address={row.from_address} />
                        </Box>
                        <Box sx={{display:'flex',flexGrow:1, gap:'8px'}}>
                          <Typography>To</Typography>
                          <L2AddressLink sx={{width:0,flexGrow:1,flexBasis:0}} address={row.to_address} />
                        </Box>
                      </Box>
                      <Box sx={{display:'flex', gap: '10px'}}>
                        <Typography variant='body2'>{+row.value / Math.pow(10,18)} ETH &gt;</Typography>
                        <Typography variant='body2' color='text.secondary'>{timeSince(row.tx_timestamp)}</Typography>
                      </Box>
                    </Box>
                  </Box>
                </TableCell>
              </TableRow>
            ))
          }
        </TableBody>
      </Table>
    </InfoCard>
  );
}

export default LatestTransactions;