import React, {useEffect} from 'react';
import InfoCard from '../../../components/InfoCard';
import {timeSince} from '../../../utils/time';
import {api} from '../../../utils/urlResolver';
import {Box, Table, TableBody, TableCell, TableHead, TableRow, Typography,} from '@mui/material';
import {atom, useRecoilState} from 'recoil';
import {MainPageAutoRefresh} from '../../../utils/consts';
import {BlockLink, TxLink} from '../../../components/Link';
import {getTypeSummary} from '../../../utils/message';

interface Tx {
  cosmos_tx_id: number;
  code: number;
  data: string;
  fee: any[];
  gas_used: string;
  gas_wanted: string;
  height: string;
  memo: string;
  messages: any[];
  raw_log: string;
  timestamp: string;
  txhash: string;
};

const txSummaryState = atom<Tx[]>({
  key: 'TxSummary',
  default: [],
});

const tableRow: Readonly<any> = {
  py: '10px',
  '&: nth-of-type(1)': {
    pt: 0,
  },
};

const tableCell: Readonly<any> = {
  px: '4px',
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

function TxSummary() {
  const [state, setState] = useRecoilState(txSummaryState);

  const reload = () => {
    (async () => {
      const res = await fetch(api('/tx/summary'));
      const json = await res.json();
      setState(json);
    })();
  };

  useEffect(() => {
    reload();
    if (MainPageAutoRefresh) {
      const id = setInterval(reload, 60000);
      return () => {
        clearInterval(id);
      };
    }
  }, []);

  return (
    <InfoCard title='LATEST TXS' buttonProps={{label: 'View all Txs', href: '/txs'}} sx={{height: '500px'}}>
      <Table>
        <TableHead>
          <TableRow>
            <TableCell align='center'>HASH</TableCell>
            <TableCell align='center'>TYPE</TableCell>
            <TableCell align='center'>HEIGHT</TableCell>
            <TableCell align='center'>TIME</TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {
            state.map((row, index) => (
              <TableRow key={index} sx={tableRow}>
                <TableCell align='center'>
                  <Box sx={{display: 'flex', minWidth: '100px'}}>
                    <TxLink sx={{width: 0, flexGrow: 1, flexBasis: 0}} hash={row.txhash}/>
                  </Box>
                </TableCell>
                <TableCell align='center'>
                  <Box sx={{display: 'flex', minWidth: '150px'}}>
                    <Typography>{getTypeSummary(row.messages)}</Typography>
                  </Box>
                </TableCell>
                <TableCell align='center'>
                  <BlockLink height={row.height}/>
                </TableCell>
                <TableCell align='center'>
                  <Typography variant='body2' color='text.secondary'>{timeSince(row.timestamp)}</Typography>
                </TableCell>
              </TableRow>
            ))
          }
        </TableBody>
      </Table>
    </InfoCard>
  );
}

export default TxSummary;
