import React from 'react';
import {
  Link,
  Typography,
} from '@mui/material';

const root = {
  display: 'flex',
  justifyContent: 'initial',
  alignItems: 'center',
  width: '154px',
  height: '19px',
  padding: '8px 28px 8px 28px',
  '&:hover': {
    bgcolor: 'rgba(0, 0, 0, 0.04)',
  },
};

function MenuItem(props: any) {
  return (
    <Link color='text.primary' href={props.href} underline='none' sx={{...root, ...props.sx}}>
      <Typography>
        {props.children}
      </Typography>
    </Link>
  );
}

export default MenuItem;