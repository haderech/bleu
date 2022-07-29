import React from 'react';
import {Box, Card, Divider, Grid, Link, Typography} from '@mui/material';
import {communityPool, inflation, latestBlock, numOfVotingProposals, stakingPool} from './Overview/state';
import {useRecoilValueLoadable} from 'recoil';

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
    <Box sx={{display: 'flex', flexDirection: 'column', alignItems: 'start', padding: 1}}>
      <Typography variant='h6' sx={{fontSize: '0.8rem', color: 'rgb(135,150,170)'}}>
        {props.title}
      </Typography>
      {
        props.content
          ? <Box sx={{display: 'flex', gap: '4px'}}>
            {props.href
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
  const inf = useRecoilValueLoadable(inflation);
  const block = useRecoilValueLoadable(latestBlock);
  const staking = useRecoilValueLoadable(stakingPool);
  const community = useRecoilValueLoadable(communityPool);
  const proposal = useRecoilValueLoadable(numOfVotingProposals);

  return (
    <Card>
      <Grid container sx={{padding: '8px 0px 8px 0px'}}>
        <Grid item lg={4} md={4} sm={6} xs={12}>
          <Box sx={outer0}>
            <TitledContent title={'HEIGHT'} content={parseInt(block.contents.height).toLocaleString()}/>
            <Divider/>
            <TitledContent title={'INFLATION'} content={(parseFloat(inf.contents.inflation) * 100).toPrecision(4)}
                           suffix={'%'}/>
          </Box>
        </Grid>
        <Grid item lg={4} md={4} sm={6} xs={12}>
          <Box sx={outer1}>
            <TitledContent title={'BONDED TOKENS'} content={parseInt(staking.contents.bonded_tokens).toLocaleString()}/>
            <Divider/>
            <TitledContent title={'UNBONDED TOKENS'}
                           content={parseInt(staking.contents.not_bonded_tokens).toLocaleString()}/>
          </Box>
        </Grid>
        <Grid item lg={4} md={4} sm={6} xs={12}>
          <Box sx={outer}>
            <TitledContent title={'COMMUNITY POOL'} content={parseInt(community.contents.amount).toLocaleString()}
                           suffix={community.contents.denom}/>
            <Divider/>
            <TitledContent title={'NUMBER OF VOTING PROPOSALS'}
                           content={parseInt(proposal.contents.num_of_voting_proposals).toLocaleString()}/>
          </Box>
        </Grid>
      </Grid>
    </Card>
  );
}

export default Overview;
