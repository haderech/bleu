import React, {useEffect} from 'react';
import InfoCard from '../../../components/InfoCard';
import {
  Table,
  TableBody,
  TableCell,
  TableFooter,
  TableHead,
  TablePagination,
  TableRow,
  Typography,
} from '@mui/material';
import {useRecoilState} from 'recoil';
import {options, state as _state} from './state';
import {useTranslation} from 'react-i18next';
import {api} from '../../../utils/urlResolver';
import {BlockLink, TxsLink} from '../../../components/Link';
import {timeSince} from '../../../utils/time';

function BlockList() {
  const {t} = useTranslation('', {useSuspense: false});
  const [state, setState] = useRecoilState(_state);
  const [opts, setOpts] = useRecoilState(options);

  const reload = (count: number, page: number) => {
    (async () => {
      const res = await fetch(api('/block', undefined, {count: count, page: page}));
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

  useEffect(() => {
    reload(opts.numRows, 1);
  }, []);

  return (
    <InfoCard title='Block' sx={{height: ''}}>
      <Table size='small'>
        <TableHead sx={{bgcolor: 'background.default'}}>
          <TableRow>
            <TableCell>{t('Height')}</TableCell>
            <TableCell>{t('Block Hash')}</TableCell>
            <TableCell>{t('Proposer')}</TableCell>
            <TableCell>{t('Txs')}</TableCell>
            <TableCell>{t('Time')}</TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {
            state.records
              ? state.records.map((row, index) => (
                <TableRow key={index}>
                  <TableCell>
                    <BlockLink height={row.height}/>
                  </TableCell>
                  <TableCell>{row.hash}</TableCell>
                  <TableCell>{row.proposer_address}</TableCell>
                  <TableCell>
                    {row.num_txs > 0 ? (<TxsLink height={row.height} num_txs={row.num_txs}/>) : (
                      <Typography>{row.num_txs}</Typography>)}
                  </TableCell>
                  <TableCell>{timeSince(row.time)}</TableCell>
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

export default BlockList;
