import React from 'react';
import {useParams} from 'react-router-dom';
import BlockDetails from './components/BlockDetails';
import ContentBody from '../../components/ContentBody';

function BlockDetailsPage() {
  const {height}: any = useParams();
  return (
    <ContentBody>
      <BlockDetails height={height} />
    </ContentBody>
  );
}

export default BlockDetailsPage;
