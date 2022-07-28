import {createTheme} from '@mui/material/styles';
import {PaletteMode} from '@mui/material';
import './theme.d';

const theme = (mode: PaletteMode = 'light') => createTheme({
  palette: {
    mode,
    ...(mode === 'light'
      ? {
          background: {
            default: '#f5f5f5',
            paper: '#fafafa',
          },
        }
      : {
          background: {
            default: '#222',
            paper: '#333',
          },
      }),
  },
  components: {
    MuiButton: {
      variants: [
        {
          props: { variant: 'text' },
          style: {
            textTransform: 'none',
            ...(mode === 'light'
              ? {
                  color: '#333',
                  fontWeight: 'normal',
                }
              : {
              }
            )
          },
        },
        {
          props: { variant: 'common' },
          style: {
            ...(mode === 'light'
            ? {
              backgroundColor: '#e7f5fa',
              color: '#0077ce',
            }
            : {
              backgroundColor: '#0077ce',
              color: '#e7f5fa',
            }),
            textTransform: 'none',
            '&:hover, &:active': {
              ...(mode === 'light'
                ? {
                  backgroundColor: '#0077ce',
                  color: '#e7f5fa',
                }
                : {
                  backgroundColor: '#e7f5fa',
                  color: '#0077ce',
                }),
            },
          },
        },
      ],
    },
    MuiTypography: {
      variants: [
        {
          props: { variant: 'h6' },
          style: {
            fontSize: '1rem',
          },
        },
        {
          props: { variant: 'body1' },
          style: {
            fontSize: '0.9rem',
          },
        },
        {
          props: { variant: 'body2' },
          style: {
            fontSize: '0.8rem',
          },
        },
        {
          props: { variant: 'mono' },
          style: {
            fontSize: '0.85rem',
            fontFamily: 'Roboto Mono',
          },
        },
      ],
    },
    MuiTableCell: {
      styleOverrides: {
        sizeSmall: {
          paddingTop: '10px',
          paddingBottom: '10px',
        },
      },
    },
    MuiTab: {
      styleOverrides: {
        root: {
          textTransform: 'none',
        },
      },
    },
  },
});

export default theme;