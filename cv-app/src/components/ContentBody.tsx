import React from 'react';
import {
  Box,
} from '@mui/material';
import {
  MainContentMaxWidth,
  MainContentPaddingHorizontal,
  MainContentPaddingTop,
  MainContentPaddingBottom,
} from '../styles/consts';

const root: Readonly<any> = {
  display: 'flex',
  flexDirection: 'column',
  alignItems: 'center',
  width: '100%',
  bgcolor: 'background.default',
};

const main: Readonly<any> = {
  width: '100%',
  maxWidth: MainContentMaxWidth,
};

const content: Readonly<any> = {
  px: MainContentPaddingHorizontal,
  pt: MainContentPaddingTop,
  pb: MainContentPaddingBottom,
};

const ContentBody = React.forwardRef((props: any, ref: any) => (
  <Box sx={{...root, ...props.sx}} ref={ref}>
    <Box sx={main}>
      <Box sx={{...content, ...props.content}}>
        {props.children}
      </Box>
    </Box>
  </Box>
));

export default ContentBody;