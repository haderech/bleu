import React from 'react';
import {useParams} from 'react-router-dom';
import Account from './components/Account';
import ContentBody from '../../components/ContentBody';

function AccountPage() {
  const {address}: any = useParams();
  return (
    <ContentBody>
      <Account address={address} />
    </ContentBody>
  );
}

export default AccountPage;