import React from 'react';
import Box from '@mui/material/Box';
import Divider from '@mui/material/Divider';
import GitHub from '@mui/icons-material/GitHub';
import Typography from '@mui/material/Typography';
import * as colors from '../colors';

const root: Readonly<any> = {
  display: 'flex',
  flexDirection: 'column',
  alignItems: 'center',
  bgcolor: 'rgb(37, 44, 52)',
  color: 'white',
  width: '100%',
  minHeight: '300px',
};

const main = {
  width: '100%',
  maxWidth: '1400px',
  paddingTop: '30px',
};

const body = {
  padding: '0px 15px 0px 15px',
};

const inner: Readonly<any> = {
  display: 'flex',
  justifyContent: 'space-between',
  flexWrap: 'wrap',
};

const innerItemFirst = {
  maxWidth: '350px',
};

const innerItem = {
  minWidth: '230px',
};

const innerItemLink = {
  fontSize: '0.8rem',
  fontWeight: 'normal',
  paddingTop: '3px',
  paddingBottom: '5px',
};

const innerItemDivider = {
  bgcolor: 'white',
  margin: '8px 0px 8px 0px',
};

const divider = {
  bgcolor: 'white',
  margin: '24px 0px 0px 0px',
};

const logoBox = {
  display: 'flex',
  alignItems: 'center',
};

const logoBoxText = {
  fontSize: '0.8rem',
  fontWeight: 'normal',
  paddingTop: '14px',
  paddingBottom: '5px',
};

const copyright = {
  display: 'flex',
  justifyContent: 'space-between',
  alignItems: 'center',
  width: '100%',
  fontSize: '0.9rem',
  paddingTop: '12px',
  paddingBottom: '8px',
};

const buttonBg = {
  display: 'flex',
  justifyContent: 'center',
  alignItems: 'center',
  borderRadius: '50%',
  width: '28px',
  height: '28px',
  bgcolor: 'rgba(255, 255, 255, 0.1)',
  color: 'white',
  '&:hover': {
    bgcolor: 'rgba(255, 255, 255, 0.9)',
    color: colors.primary,
  },
};

const button = {
  width: '14px',
  height: '14px',
};

const Logo = () => {
  return (
    <svg version="1.1" xmlns="http://www.w3.org/2000/svg" xmlnsXlink="http://www.w3.org/1999/xlink" width="30" height="30">
      <rect x='0' y='0' width='30' height='30' style={{fill: '#555'}} />
      <rect x='7' y='5' width='16' height='3' style={{fill: '#fefefe'}} />
      <rect x='7' y='10' width='16' height='3' style={{fill: '#fefefe'}} />
      <rect x='13' y='10' width='4' height='15' style={{fill: '#fefefe'}} />
    </svg>
  );
};

export default function Footer() {
  return (
    <Box sx={root}>
      <Box sx={main}>
        <Box sx={body}>
          <Box sx={inner}>
            <Box sx={innerItemFirst}>
              <Box sx={logoBox}>
                <Logo /><Typography sx={{paddingLeft: '10px'}}>Powered by Turnpike</Typography>
              </Box>
              <Typography sx={logoBoxText}>BLEU is a set of tools to build a blockchain explorer for any protocols.</Typography>
            </Box>
            <Box sx={innerItem}>
              <Typography>Column 1</Typography>
              <Divider sx={innerItemDivider} />
              <Typography sx={innerItemLink}>item 1</Typography>
              <Typography sx={innerItemLink}>item 2</Typography>
              <Typography sx={innerItemLink}>item 3</Typography>
              <Typography sx={innerItemLink}>item 4</Typography>
              <Typography sx={innerItemLink}>item 5</Typography>
            </Box>
            <Box sx={innerItem}>
              <Typography>Column 2</Typography>
              <Divider sx={innerItemDivider} />
            </Box>
            <Box sx={innerItem}>
              <Typography>Column 3</Typography>
              <Divider sx={innerItemDivider} />
            </Box>
          </Box>
          <Divider variant='middle' light={true} sx={divider} />
          <Box sx={copyright}>
            <Typography sx={{fontSize: '0.9rem'}}>BLEU &copy; 2021 Turnpike</Typography>
            <a href='https://github.com/turnpike/bleu' target='_blank'>
              <Box sx={buttonBg}>
                <GitHub sx={button} />
              </Box>
            </a>
          </Box>
        </Box>
      </Box>
    </Box>
  );
};
