import React from 'react';
import InfoCard from '../../../components/InfoCard';
import { timeSince } from '../../../utils/time';
import { api } from '../../../utils/urlResolver';
import {
  Avatar,
  Box,
  Grid,
  Link,
  Table,
  TableBody,
  TableCell,
  TableRow,
  Typography,
} from '@mui/material';
import { Loadable, selector, useRecoilValueLoadable } from 'recoil';
import { BlockLink } from '../../../components/Link';

interface Block {
  block_number: string;
  hash: string;
  txn: string;
  block_timestamp: string;
};

const latestBlocksState = selector({
  key: 'LatestBlocks',
  get: async () => {
    const res = await fetch(api('/blocks/latest'));
    const blocks = await res.json();
    console.log(blocks)
    return blocks;
  },
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

function LatestBlocks() {
  const latestBlocks: Loadable<Block[]> = useRecoilValueLoadable(latestBlocksState);
  return (
    <InfoCard title='Latest Blocks' buttonProps={{ label: 'View all blocks', href: '/blocks' }} sx={{ height: '500px' }}>
      <Table>
        <TableBody>
          {
            latestBlocks.state === 'hasValue'
              ? latestBlocks.contents.map((row, index) => (
                <TableRow key={index} sx={tableRow}>
                  <TableCell sx={(index === latestBlocks.contents.length - 1) ? tableCellLast : tableCell}>
                    <Grid container spacing={1}>
                      <Grid item lg={6} md={6} sm={12} xs={12} sx={content}>
                        <Box sx={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
                          <Avatar variant='rounded'>Bk</Avatar>
                          <Grid container>
                            <Grid item lg={12} md={12} sm={2} xs={2}>
                              <BlockLink param={row.block_number} />
                            </Grid>
                            <Grid item lg={12} md={12} sm={10} xs={10}>
                              <Typography variant='body2' color='text.secondary'>{timeSince(row.block_timestamp)}</Typography>
                            </Grid>
                          </Grid>
                        </Box>
                      </Grid>
                      <Grid item lg={6} md={6} sm={12} xs={12} sx={content}>
                        <Box sx={{ display: 'flex', p: '0px', gap: '8px' }}>
                          <Typography>Hash</Typography>
                          <BlockLink sx={{ width: 0, flexGrow: 1, flexBasis: 0 }} param={row.hash} />
                        </Box>
                        <Typography variant='body2'>{row.txn} txns</Typography>
                      </Grid>
                    </Grid>
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

export default LatestBlocks;