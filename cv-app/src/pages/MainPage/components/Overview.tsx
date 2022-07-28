import React from 'react';
import {
  Box,
  Card,
  Divider,
  Grid,
  Link,
  Typography
} from '@mui/material';
import {wrappedEth, summary} from './Overview/state';
import {useRecoilState, useRecoilValueLoadable} from 'recoil';
import {toEther} from '../../../utils/ethUtils';

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
    <Box sx={{display:'flex',flexDirection:'column',alignItems:'start',padding:1}}>
      <Typography variant='h6' sx={{fontSize:'0.8rem', color:'rgb(135,150,170)'}}>
        {props.title}
      </Typography>
      {
        props.content
        ? <Box sx={{display:'flex',gap:'4px'}}>
            { props.href
              ? <Link variant='h6' underline='none' sx={{fontSize: '1rem'}} href={props.href}>
                  {props.content || 'N/A'}
                </Link>
              : <Typography variant='h6'>{props.content}</Typography>
            }
            <Typography variant='h6'>
              {props.suffix}
            </Typography>
          </Box>
        : null
      }
    </Box>
  );
}

function Overview() {
  const weth = useRecoilValueLoadable(wrappedEth);
  const sum = useRecoilValueLoadable(summary);

  return (
    <Card>
      <Grid container sx={{padding: '8px 0px 8px 0px'}}>
        <Grid item lg={4} md={4} sm={6} xs={12}>
          <Box sx={outer0}>
            <TitledContent title={'WRAPPED ETHER'} content={toEther(weth.contents)} suffix='ETH' />
            <Divider />
            <TitledContent title={'TRANSACTIONS'} content={sum.contents.tx_count} suffix='TXs' href={'/txs'} />
            <Divider sx={{display:{xs:'block', sm:'none'}}} />
          </Box>
        </Grid>
        <Grid item lg={4} md={4} sm={6} xs={12}>
          <Box sx={outer1}>
            <TitledContent title={'LATEST TRANSACTION BATCH INDEX'} content={sum.contents.latest_tx_batch_index} href={'/blocks'} />
            <Divider />
            <TitledContent title={'LATEST STATE BATCH INDEX'} content={sum.contents.latest_state_batch_index} href={'/blocks?isState=true'} />
          </Box>
        </Grid>
        <Grid item lg={4} md={4} sm={12} xs={12}>
          <Box sx={outer}>
            <Divider sx={{display:{sm:'block', md:'none'}, paddingTop: {xs:0, sm:1}}} />
            <TitledContent title={'RESERVED AREA'} content={undefined} />
          </Box>
        </Grid>
      </Grid>
    </Card>
  );
}

export default Overview;