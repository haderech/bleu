import React from 'react';
import {
  Box,
  Button,
  Divider,
  Paper,
  Typography,
} from '@mui/material';
import {useTranslation} from 'react-i18next';
import {
  InfoCardContentPadding,
} from '../styles/consts';

const root: Readonly<any> = {
  display: 'flex',
  flexDirection: 'column',
};

const header: Readonly<any> = {
  display: 'flex',
  px: '16px',
  py: '12px',
  gap: '8px',
  flexShrink: 0,
};

const content: Readonly<any> = {
  mx: InfoCardContentPadding,
  my: InfoCardContentPadding,
  overflowY: 'auto',
  flexShrink: 1,
  flexGrow: 1,
};

const footer: Readonly<any> = {
  display: 'flex',
  justifyContent: 'center',
  padding: '10px',
  flexShrink: 0,
};

const footerButton: Readonly<any> = {
  flexGrow: 1,
  fontSize: '0.8rem',
  py: '5px',
};

function BottomButton(props: any) {
  if (props.buttonProps !== undefined) {
    const {t} = useTranslation('', {useSuspense: false});
    return (
      <React.Fragment>
        <Divider />
        <Box sx={footer} >
          <Button variant='common' sx={footerButton} href={props.buttonProps.href}>{t(props.buttonProps.label)}</Button>
        </Box>
      </React.Fragment>
    );
  } else {
    return null;
  }
}

function InfoCard(props: any) {
  const {t} = useTranslation('', {useSuspense: false});
  return (
    <Paper sx={{...root, ...props.sx}}>
      {props.head ? props.head : (
        <React.Fragment>
          <Box sx={header}>
            <Typography variant='h6'>{t(props.title)}</Typography>
            {props.subtitle
              ? <Typography variant='h6' color='text.secondary' sx={{fontWeight:'normal'}}>{props.subtitle}</Typography>
              : null
            }
          </Box>
          <Divider />
        </React.Fragment>
      )}
      <Box sx={{...content, ...props.contentProps}}>
        {props.children}
      </Box>
      <BottomButton buttonProps={props.buttonProps} />
    </Paper>
  );
}

export default InfoCard;