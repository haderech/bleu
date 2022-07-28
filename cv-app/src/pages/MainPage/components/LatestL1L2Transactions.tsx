import React, {useEffect} from 'react';
import InfoCard from '../../../components/InfoCard';
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
import {L1BlockLink, L1TransactionLink, L2TransactionLink} from '../../../components/Link';
import {api} from '../../../utils/urlResolver';
import {MainPageAutoRefresh} from '../../../utils/consts';

interface L1L2Transaction {
  l1_block_number: string;
  l1_tx_hash: string;
  l2_tx_hash: string;
}

const latestL1L2TransactionsState = atom<L1L2Transaction[]>({
  key: 'LatestL1L2Transactions',
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

function LatestL1L2Transactions() {
  const [state, setState] = useRecoilState(latestL1L2TransactionsState);

  const reload = () => {
    (async () => {
      const res = await fetch(api('/tx/l1tol2/latest'));
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
    <InfoCard title='Latest L1→L2 Transactions' buttonProps={{label:'View all L1→L2 transactions',href:'/l1l2txs'}} sx={{height:'500px'}}>
      <Table>
        <TableBody>
          {
            state.map((row, index) => (
              <TableRow key={index} sx={tableRow}>
                <TableCell sx={(index === state.length - 1) ? tableCellLast : tableCell}>
                  <Box sx={{display:'flex',alignItems:'center',gap:'12px',width:'100%'}}>
                    <Avatar>Tx</Avatar>
                    <Box sx={{flexGrow:1,flexBasis:0,width:0}}>
                      <Box sx={{display:'flex',gap:'4px'}}>
                        <Typography>Block#</Typography>
                        <L1BlockLink blockNumber={row.l1_block_number} />
                      </Box>
                      <Box sx={{display:'flex'}}>
                        <Box sx={{display:'flex',flexGrow:1, gap:'8px'}}>
                          <Typography>L1 Tx#</Typography>
                          <L1TransactionLink sx={{width:0,flexGrow:1,flexBasis:0}} hash={row.l1_tx_hash} />
                        </Box>
                        <Box sx={{display:'flex',flexGrow:1, gap:'8px'}}>
                          <Typography>L2 Tx#</Typography>
                          <L2TransactionLink sx={{width:0,flexGrow:1,flexBasis:0}} hash={row.l2_tx_hash} />
                        </Box>
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

export default LatestL1L2Transactions;