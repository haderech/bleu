import React from 'react';
import {useParams} from 'react-router-dom';
import ValidatorDetails from './components/ValidatorDetails';
import ContentBody from '../../components/ContentBody';

function ValidatorDetailsPage() {
  const {address}: any = useParams();
  return (
    <ContentBody>
      <ValidatorDetails address={address}/>
    </ContentBody>
  );
}

export default ValidatorDetailsPage;
