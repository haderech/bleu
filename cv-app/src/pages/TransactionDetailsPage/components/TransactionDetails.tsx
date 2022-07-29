import React, {useEffect} from 'react';
import {useParams} from 'react-router-dom';
import PropTypes from 'prop-types';
import {useRecoilState} from 'recoil';
import InfoCard from '../../../components/InfoCard';
import {
  Box,
  Tab,
  Tabs,
} from '@mui/material';
import {options} from './state';
import Overview from './Overview';

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
      id={`transaction-details-tabpanel-${index}`}
      aria-labelledby={`transaction-details-tab-${index}`}
      {...other}
      sx={{px:'12px'}}
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
    id: `transaction-details-tab-${index}`,
    'aria-controls': `transaction-details-tabpanel-${index}`,
  };
}

function TransactionDetails(props: any) {
  const {txHash}: any = useParams();
  const [opts, setOpts] = useRecoilState(options);

  useEffect(() => {
    setOpts({
      ...opts,
      txHash: txHash,
    });
  }, []);

  const handleChange = (event: any, newValue: any) => {
    setOpts({
      ...opts,
      index: newValue,
    });
  };

  return (
    <InfoCard title='Transaction Details' contentProps={{m:0}}>
      <Box sx={cardHeaderC1}>
        <Tabs value={opts.index} onChange={handleChange} aria-label='transaction-details-tabs'>
          <Tab label='Overview' {...a11yProps(0)} />
          <Tab label='Comments' {...a11yProps(2)} />
        </Tabs>
      </Box>
      <TabPanel value={opts.index} index={0}>
        <Overview />
      </TabPanel>
    </InfoCard>
  );
}

export default TransactionDetails;
