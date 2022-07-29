import React, {useEffect} from 'react';
import InfoCard from '../../../components/InfoCard';
import {timeSince} from '../../../utils/time';
import {api} from '../../../utils/urlResolver';
import {Table, TableBody, TableCell, TableHead, TableRow, Typography,} from '@mui/material';
import {atom, useRecoilState} from 'recoil';
import {MainPageAutoRefresh} from '../../../utils/consts';
import {BlockLink, TxsLink} from '../../../components/Link';

interface Block {
  cosmos_block_id: number;
  app_hash: string;
  chain_id: string;
  consensus_hash: string;
  data_hash: string;
  evidence_hash: string;
  hash: string;
  height: string;
  last_block_id: string;
  last_commit_hash: string;
  last_results_hash: string;
  next_validators_hash: string;
  num_txs: number;
  proposer_address: string;
  time: string;
  validators_hash: string;
  version: string;
};

const blockSummaryState = atom<Block[]>({
  key: 'BlockSummary',
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

function BlockSummary() {
  const [state, setState] = useRecoilState(blockSummaryState);

  const reload = () => {
    (async () => {
      const res = await fetch(api('/block/summary'));
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
    <InfoCard title='LATEST BLOCKS' buttonProps={{label: 'View all Blocks', href: '/blocks'}} sx={{height: '500px'}}>
      <Table>
        <TableHead>
          <TableRow>
            <TableCell align='center'>HEIGHT</TableCell>
            <TableCell align='center'>PROPOSER</TableCell>
            <TableCell align='center'>TXS</TableCell>
            <TableCell align='center'>TIME</TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {
            state.map((row, index) => (
              <TableRow key={index} sx={tableRow}>
                <TableCell align='center'>
                  <BlockLink height={row.height}/>
                </TableCell>
                <TableCell align='center'>
                  <Typography>{row.proposer_address}</Typography>
                </TableCell>
                <TableCell align='center'>
                  {row.num_txs > 0 ? (<TxsLink height={row.height} num_txs={row.num_txs}/>) : (
                    <Typography>{row.num_txs}</Typography>)}
                </TableCell>
                <TableCell align='center'>
                  <Typography variant='body2' color='text.secondary'>{timeSince(row.time)}</Typography>
                </TableCell>
              </TableRow>
            ))
          }
        </TableBody>
      </Table>
    </InfoCard>
  );
}

export default BlockSummary;
