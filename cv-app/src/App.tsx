import React from 'react';
import {BrowserRouter as Router, Route, Switch} from 'react-router-dom';
import {RecoilRoot} from 'recoil';
import Box from '@mui/material/Box';
import {ThemeProvider} from '@mui/material/styles';
import theme from './styles/theme';
import './App.css';
import './i18n';
import Header from './components/Header';
import Footer from './components/Footer';
import MainPage from './pages/MainPage';
import BlockDetailsPage from './pages/BlockDetailsPage/BlockDetailsPage';
import BlocksPage from './pages/BlocksPage';
import TransactionDetailsPage from './pages/TransactionDetailsPage';
import TransactionsPage from './pages/TransactionsPage/TransactionsPage';
import ProposalsPage from './pages/ProposalsPage/ProposalsPage';
import ProposalDetailsPage from './pages/ProposalDetailsPage/ProposalDetailsPage';
import ValidatorsPage from './pages/ValidatorsPage/ValidatorsPage';
import ValidatorDetailsPage from './pages/ValidatorDetailsPage/ValidatorDetailsPage';

const root: Readonly<any> = {
  display: 'flex',
  flexDirection: 'column',
  justifyContent: 'space-between',
  bgcolor: 'background.default',
  minHeight: '100vh',
};

const main: Readonly<any> = {
  display: 'flex',
  flexDirection: 'column',
  alignItems: 'center',
};

function App() {
  return (
    <RecoilRoot>
      <ThemeProvider theme={theme()}>
        <Box sx={root}>
          <Box sx={main}>
            <Header/>
            <Router>
              <Switch>
                <Route path='/blocks'>
                  <BlocksPage/>
                </Route>
                <Route path={'/block/height/:height'}>
                  <BlockDetailsPage/>
                </Route>
                <Route path={'/txs/height/:height'}>
                  <TransactionsPage/>
                </Route>
                <Route exact path={'/txs'}>
                  <TransactionsPage/>
                </Route>
                <Route path={'/tx/hash/:txHash'}>
                  <TransactionDetailsPage/>
                </Route>
                <Route path='/proposals'>
                  <ProposalsPage/>
                </Route>
                <Route path='/proposal/id/:id'>
                  <ProposalDetailsPage/>
                </Route>
                <Route path='/validators'>
                  <ValidatorsPage/>
                </Route>
                <Route path='/validator/address/:address'>
                  <ValidatorDetailsPage/>
                </Route>
                <Route path='/'>
                  <MainPage/>
                </Route>
              </Switch>
            </Router>
          </Box>
          <Footer/>
        </Box>
      </ThemeProvider>
    </RecoilRoot>
  );
}

export default App;
