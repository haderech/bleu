import React from 'react';
import {useParams} from 'react-router-dom';
import BlockDetails from './components/BlockDetails';
import ContentBody from '../../components/ContentBody';

function BlockDetailsPage() {
  const {blockNumber}: any = useParams();
  return (
    <ContentBody>
      <BlockDetails blockNumber={blockNumber} />
    </ContentBody>
  );
}

export default BlockDetailsPage;