import React, { useEffect } from 'react';
import { useParams } from 'react-router-dom';
import PropTypes from 'prop-types';
import { useRecoilState } from 'recoil';
import {
  Box,
  Tab,
  Tabs,
} from '@mui/material';
import InfoCard from '../../../components/InfoCard';
import Overview from './Overview';
import { options } from './state';

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
      id={`block-details-tabpanel-${index}`}
      aria-labelledby={`block-details-tab-${index}`}
      {...other}
      sx={{ px: '12px', pb: '12px' }}
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
    id: `block-details-tab-${index}`,
    'aria-controls': `block-details-tabpanel-${index}`,
  };
}

function BlockDetails(props: any) {
  const { blockNumber }: any = useParams();
  const [opts, setOpts] = useRecoilState(options);

  useEffect(() => {
    if (opts.blockNumber !== blockNumber) {
      setOpts({
        ...opts,
        blockNumber: blockNumber,
      });
    }
  });

  const handleChange = (event: any, newValue: any) => {
    setOpts({
      ...opts,
      index: newValue,
    });
  };

  return (
    <InfoCard title='Block' subtitle={`#${blockNumber}`} contentProps={{ m: 0 }}>
      <Box sx={cardHeaderC1}>
        <Tabs value={opts.index} onChange={handleChange} aria-label='block-details-tabs'>
          <Tab label='Overview' {...a11yProps(0)} />
        </Tabs>
      </Box>
      <TabPanel value={opts.index} index={0}>
        <Overview />
      </TabPanel>
    </InfoCard>
  );
}

export default BlockDetails;