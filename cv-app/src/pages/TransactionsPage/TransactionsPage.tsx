import React from 'react';
import ContentBody from '../../components/ContentBody';
import TransactionsList from './components/TransactionsList';
import {useParams} from 'react-router-dom';

function TransactionsPage() {
  const {height}: any = useParams();
  return (
    <ContentBody>
      <TransactionsList height={height}/>
    </ContentBody>
  );
}

export default TransactionsPage;
