import React from 'react';
import {useParams} from 'react-router-dom';
import BlockDetails from './components/BlockDetails';
import ContentBody from '../../components/ContentBody';

function BlockDetailsPage() {
  const {param}: any = useParams();
  return (
    <ContentBody>
      <BlockDetails param={param} />
    </ContentBody>
  );
}

export default BlockDetailsPage;