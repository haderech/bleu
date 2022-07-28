import React, {useEffect} from 'react';
import {useParams} from 'react-router-dom';
import PropTypes from 'prop-types';
import {useRecoilState, useRecoilValueLoadable} from 'recoil';
import InfoCard from '../../../components/InfoCard';
import {
  Box,
  Card,
  Grid,
  Tab,
  Table,
  TableBody,
  TableCell,
  TableRow,
  Tabs,
  Typography
} from '@mui/material';
import {options, balance as _balance, tabIndex as _tabIndex} from './state';
import {toEther} from '../../../utils/ethUtils';
import Transactions from './Transactions';

const cardHeader: Readonly<any> = {
  display: 'flex',
  flexDirection: 'row',
  justifyContent: 'space-between',
  height: '50px',
};

const cardHeaderC1: Readonly<any> = {
  borderBottom: 1,
  borderColor: 'divider',
};

function TabPanel(props: any) {
  const { children, value, index, ...other } = props;

  return (
    <Box
      role='tabpanel'
      hidden={value !== index}
      id={`account-tabpanel-${index}`}
      aria-labelledby={`account-tab-${index}`}
      {...other}
    >
      {value === index && (
        <React.Fragment>
          {children}
        </React.Fragment>
      )}
    </Box>
  );
}

TabPanel.propTypes = {
  children: PropTypes.node,
  index: PropTypes.number.isRequired,
  value: PropTypes.number.isRequired,
};

function a11yProps(index: number) {
  return {
    id: `account-tab-${index}`,
    'aria-controls': `account-tabpanel-${index}`,
  };
}

function Account(props: any) {
  const {address}: any = useParams();
  const [opts, setOpts] = useRecoilState(options);
  const [tabIndex, setTabIndex] = useRecoilState(_tabIndex);
  const balance = useRecoilValueLoadable(_balance);

  useEffect(() => {
    setOpts({
      ...opts,
      address: address,
    });
  }, []);

  const handleChange = (event: any, newValue: any) => {
    setTabIndex(newValue);
  };

  return (
    <Grid container spacing={2}>
      <Grid item lg={6} md={6} sm={12} xs={12}>
        <InfoCard title={`Address ${address}`} sx={{height:'100%'}} contentProps={{mt:0,mb:0}}>
          <Table>
            <TableBody>
              <TableRow>
                <TableCell sx={{borderBottom:'none'}}>
                  <Grid container>
                    <Grid item lg={4} md={4} sm={12} xs={12}>
                      <Typography variant='body1'>Balance:</Typography>
                    </Grid>
                    <Grid item lg={8} md={8} sm={12} xs={12}>
                      {
                        balance.state === 'hasValue' ? <Typography>{toEther(balance.contents)} Ether</Typography> : null
                      }
                    </Grid>
                  </Grid>
                </TableCell>
              </TableRow>
            </TableBody>
          </Table>
        </InfoCard>
      </Grid>
      <Grid item lg={6} md={6} sm={12} xs={12}>
        <Card>
          <Box sx={cardHeader}>
            <Box sx={cardHeaderC1}>
            </Box>
          </Box>
        </Card>
      </Grid>
      <Grid item lg={12} md={12} sm={12} xs={12}>
        <InfoCard head={(
          <Box sx={cardHeaderC1}>
            <Tabs value={tabIndex} onChange={handleChange} aria-label='account-tabs'>
              <Tab label="Transactions" {...a11yProps(0)} />
              <Tab label="ERC20 Token Txs" {...a11yProps(2)} />
              <Tab label="Comments" {...a11yProps(5)} />
            </Tabs>
          </Box>
        )}>
          <TabPanel value={tabIndex} index={0}>
            <Transactions address={address} />
          </TabPanel>
        </InfoCard>
      </Grid>
    </Grid>
  );
}

export default Account;