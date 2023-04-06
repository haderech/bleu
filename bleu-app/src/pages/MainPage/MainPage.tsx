import React from 'react';
import {
  Box,
} from '@mui/material';
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
      </Box>
    </Box>
  );
}

export default MainPage;
