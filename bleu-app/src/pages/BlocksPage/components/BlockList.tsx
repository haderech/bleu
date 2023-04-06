import React, { useEffect } from 'react';
import { useLocation } from 'react-router-dom';
import InfoCard from '../../../components/InfoCard';
import { timeSince } from '../../../utils/time';
import {
  Link,
  Table,
  TableBody,
  TableFooter,
  TableHead,
  TablePagination,
  TableRow,
  TableCell,
  Typography,
} from '@mui/material';
import { useRecoilState } from 'recoil';
import { options, state as _state } from './state';
import { BlockLink, BlockTxnLink } from '../../../components/Link';
import { useTranslation } from 'react-i18next';
import { api } from '../../../utils/urlResolver';

function BlockList() {
  const { t } = useTranslation('', { useSuspense: false });
  const [state, setState] = useRecoilState(_state);
  const [opts, setOpts] = useRecoilState(options);
  const { search } = useLocation();

  const reload = (count: number, page: number) => {
    (async () => {
      const res = await fetch(api('/blocks', undefined, { count: count, page: page }));
      const json = await res.json();
      setState(json);
    })();
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

  return (
    <InfoCard title='Blocks' sx={{ height: '' }}>
      <Table size='small'>
        <TableHead sx={{ bgcolor: 'background.default' }}>
          <TableRow>
            <TableCell>{t('Block')}</TableCell>
            <TableCell>
              <Link sx={{ fontWeight: 'inherit' }} component='button' underline='none' onClick={toggleTimestamp}>
                {opts.datetime ? t('Date Time (UTC)') : t('Age')}
              </Link>
            </TableCell>
            <TableCell>{t('Txn')}</TableCell>
            <TableCell>{t('Hash')}</TableCell>
            <TableCell>{t('Gas Used')}</TableCell>
            <TableCell>{t('Gas Limit')}</TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {
            state.records
              ? state.records.map((row, index) => (
                <TableRow key={index}>
                  <TableCell><BlockLink blockNumber={row.block_number} /></TableCell>
                  <TableCell>
                    <Typography noWrap={true}>
                      {opts.datetime ? new Date(+row.block_timestamp * 1000).toLocaleString() : timeSince(row.block_timestamp)}
                    </Typography>
                  </TableCell>
                  <TableCell>{row.txn ? <BlockTxnLink blockNumber={row.block_number} txn={row.txn}/> : row.txn}</TableCell>
                  <TableCell>{row.hash}</TableCell>
                  <TableCell>{row.gas_used}</TableCell>
                  <TableCell>{row.gas_limit}</TableCell>
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
                  colSpan={5}
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
                  sx={{ borderBottom: 'none' }}
                />
                : null
            }
          </TableRow>
        </TableFooter>
      </Table>
    </InfoCard>
  );
}

export default BlockList;