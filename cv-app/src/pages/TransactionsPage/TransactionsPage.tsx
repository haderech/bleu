import React from 'react';
import ContentBody from '../../components/ContentBody';
import TransactionsList from './components/TransactionsList';

function TransactionsPage() {
  return (
    <ContentBody>
      <TransactionsList />
    </ContentBody>
  );
}

export default TransactionsPage;
