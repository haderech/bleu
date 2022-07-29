import React, {useEffect} from 'react';
import {useRecoilState} from 'recoil';
import {useTranslation} from 'react-i18next';
import {
  Box,
  Table,
  TableBody,
  TableCell,
  TableFooter,
  TableHead,
  TablePagination,
  TableRow,
  Typography,
} from '@mui/material';
import InfoCard from '../../../components/InfoCard';
import {BlockLink, TxLink} from '../../../components/Link';
import {options, state as _state} from './state';
import {api} from '../../../utils/urlResolver';
import {getTypeSummary} from '../../../utils/message';

function TransactionsList(props: any) {
  const {t} = useTranslation('', {useSuspense: false});
  const [state, setState] = useRecoilState(_state);
  const [opts, setOpts] = useRecoilState(options);

  const height = props.height;
  const reload = (count: number, page: number) => {
    if (!height) {
      (async () => {
        const res = await fetch(api('/tx', undefined, {count: count, page: page}));
        const json = await res.json();
        setState(json);
      })();
    } else {
      (async () => {
        const res = await fetch(api('/tx/height', height, {count: count, page: page}));
        const json = await res.json();
        setState(json);
      })();
    }
  };
  const handleChangePage = (event: any, newPage: any) => {
    reload(opts.numRows, newPage + 1);
  };
  const handleChangeRowsPerPage = (event: any) => {
    const page = Math.floor(((state.page_info.page - 1) * opts.numRows) / +event.target.value);
    setOpts({
      ...opts,
      numRows: +event.target.value,
    });
    reload(+event.target.value, page + 1);
  };
  const toggleTimestamp = () => {
    setOpts({
      ...opts,
      datetime: !opts.datetime,
    });
  };

  useEffect(() => {
    reload(opts.numRows, 1);
  }, []);

  const getTitle = () => {
    if (!height) {
      return 'Transactions';
    } else {
      return 'Transactions in Block';
    }
  };

  return (
    <InfoCard title={getTitle()} subtitle={height ? `#${height}` : null} sx={{height: ''}}>
      <Table size='small'>
        <TableHead sx={{bgcolor: 'background.default'}}>
          <TableRow>
            <TableCell>{t('Tx Hash')}</TableCell>
            <TableCell>{t('Type')}</TableCell>
            <TableCell>{t('Result')}</TableCell>
            <TableCell>{t('Fee')}</TableCell>
            <TableCell>{t('Height')}</TableCell>
            <TableCell>{t('Timestamp')}</TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {
            state.records
              ? state.records.map((row, index) => (
                <TableRow key={index}>
                  <TableCell>
                    <Box sx={{display: 'flex', minWidth: '150px'}}>
                      <TxLink sx={{width: 0, flexGrow: 1, flexBasis: 0}} hash={row.txhash}/>
                    </Box>
                  </TableCell>
                  <TableCell>
                    <Typography>{getTypeSummary(row.messages)}</Typography>
                  </TableCell>
                  <TableCell>
                    <Typography>{row.code == 0 ? 'success' : 'fail'}</Typography>
                  </TableCell>
                  <TableCell>
                    <Typography>{row.fee.length > 0 ? `${row.fee[0]['amount']}${row.fee[0]['denom']}` : '0uatom'}</Typography>
                  </TableCell>
                  <TableCell>
                    <BlockLink height={row.height}/>
                  </TableCell>
                  <TableCell>
                    <Typography>{row.timestamp}</Typography>
                  </TableCell>
                </TableRow>
              ))
              : null
          }
        </TableBody>
        <TableFooter>
          <TableRow>
            {
              state.page_info
                ? <TablePagination
                  rowsPerPageOptions={[10, 25, 50, 100]}
                  colSpan={9}
                  count={state.page_info.total_count}
                  rowsPerPage={opts.numRows}
                  page={state.page_info.page - 1}
                  SelectProps={{
                    inputProps: {
                      'aria-label': 'rows per page',
                    },
                    native: true,
                  }}
                  onPageChange={handleChangePage}
                  onRowsPerPageChange={handleChangeRowsPerPage}
                  showFirstButton={true}
                  showLastButton={true}
                  sx={{borderBottom: 'none'}}
                />
                : null
            }
          </TableRow>
        </TableFooter>
      </Table>
    </InfoCard>
  );
}

export default TransactionsList;
