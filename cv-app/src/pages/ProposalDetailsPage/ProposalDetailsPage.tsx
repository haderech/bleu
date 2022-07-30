import React from 'react';
import {useParams} from 'react-router-dom';
import ProposalDetails from './components/ProposalDetails';
import ContentBody from '../../components/ContentBody';

function ProposalDetailsPage() {
  const {height}: any = useParams();
  return (
    <ContentBody>
      <ProposalDetails height={height} />
    </ContentBody>
  );
}

export default ProposalDetailsPage;
