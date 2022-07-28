import React from 'react';
import {
  Box,
  Grid,
} from '@mui/material';
import LatestL1Batches from './components/LatestL1Batches';
import LatestTransactions from './components/LatestTransactions';
import LatestL1L2Transactions from './components/LatestL1L2Transactions';
import Overview from './components/Overview';
import SearchBar from './components/SearchBar';

const root: Readonly<any> = {
  display: 'flex',
  flexDirection: 'column',
  alignItems: 'center',
  width: '100%',
  height: '100%',
};

const main = {
  width: '100%',
  maxWidth: 1400,
  zIndex: 10,
};

const body = {
  padding: '0px 15px 100px 15px',
};

const placeholder = {
  display: 'flex',
  justifyContent: 'center',
  alignItems: 'center',
  height: '218px',
};

const band: Readonly<any> = {
  bgcolor: 'rgb(37, 44, 52)',
  height: '268px',
  width: '100vw',
  zIndex: 0,
  position: 'absolute',
};

function MainPage() {
  return (
    <Box sx={root}>
      <Box sx={band} />
      <Box sx={main}>
        <Box sx={placeholder}>
          <SearchBar />
        </Box>
        <Grid container spacing={2} sx={body}>
          <Grid item lg={12} md={12} sm={12} xs={12}>
            <Overview />
          </Grid>
          <Grid item lg={4} md={4} sm={12} xs={12}>
            <LatestL1Batches />
          </Grid>
          <Grid item lg={4} md={4} sm={12} xs={12}>
            <LatestTransactions />
          </Grid>
          <Grid item lg={4} md={4} sm={12} xs={12}>
            <LatestL1L2Transactions />
          </Grid>
        </Grid>
      </Box>
    </Box>
  );
}

export default MainPage;
