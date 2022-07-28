import React from 'react';
import {useParams} from 'react-router-dom';
import TransactionDetails from './components/TransactionDetails';
import ContentBody from '../../components/ContentBody';

function TransactionDetailsPage() {
  const {txHash}: any = useParams();
  return (
    <ContentBody>
      <TransactionDetails txHash={txHash} />
    </ContentBody>
  );
}

export default TransactionDetailsPage;