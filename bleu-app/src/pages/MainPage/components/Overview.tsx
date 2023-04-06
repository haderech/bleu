import React from 'react';
import {
  Box,
  Card,
  Divider,
  Grid,
  Typography
} from '@mui/material';
import { summary } from './Overview/state';
import { useRecoilState, useRecoilValueLoadable } from 'recoil';
import { toEther } from '../../../utils/ethUtils';

const outer = {
  borderRightColor: '#e0e0e0 !important',
  padding: '0px 8px 0px 8px',
};

const outer0 = {
  ...outer,
  borderRight: {
    xs: 0,
    sm: 1,
  },
};

const outer1 = {
  ...outer,
  borderRight: {
    md: 1,
  },
};

function TitledContent(props: any) {
  return (
    <Box sx={{ display: 'flex', flexDirection: 'column', alignItems: 'start', padding: 1 }}>
      <Typography variant='h6' sx={{ fontSize: '0.8rem', color: 'rgb(135,150,170)' }}>
        {props.title}
      </Typography>
      <Typography variant='h6' sx={{ fontSize: '1rem', color: (props.content ? 'text.primary' : 'background.paper') }}>
        {props.content || 'N/A'} {props.suffix}
      </Typography>
    </Box>
  );
}

function Overview() {
  const sum = useRecoilValueLoadable(summary);

  return (
    <Card>
      <Grid container sx={{ padding: '8px 0px 8px 0px' }}>
        <Grid item lg={4} md={4} sm={6} xs={12}>
          <Box sx={outer0}>
            <TitledContent title={'LATEST BLOCK'} content={sum.contents.latest_block_number} />
          </Box>
        </Grid>
        <Grid item lg={4} md={4} sm={6} xs={12}>
          <Box sx={outer1}>
            <TitledContent title={'TRANSACTIONS'} content={sum.contents.total_transaction_count} />
          </Box>
        </Grid>
        <Grid item lg={4} md={4} sm={12} xs={12}>
          <Box sx={outer}>
            <TitledContent title={'LATEST BLOCK GAS'} content={sum.contents.latest_block_gas_used} />
          </Box>
        </Grid>
      </Grid>
    </Card>
  );
}

export default Overview;