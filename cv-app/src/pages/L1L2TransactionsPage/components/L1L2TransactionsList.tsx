import React, {useEffect} from 'react';
import {useRecoilState} from 'recoil';
import {useTranslation} from 'react-i18next';
import {
  Box,
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
import InfoCard from '../../../components/InfoCard';
import {L1AddressLink, L1BlockLink, L1TransactionLink, L2TransactionLink} from '../../../components/Link';
import {timeSince} from '../../../utils/time';
import {options, state as _state} from './state';
import {api} from '../../../utils/urlResolver';

function L1L2TransactionsList() {
  const {t} = useTranslation('', {useSuspense: false});
  const [state, setState] = useRecoilState(_state);
  const [opts, setOpts] = useRecoilState(options);

  const reload = (count: number, page: number) => {
    (async () => {
      const res = await fetch(api('/tx/l1tol2', undefined, {count: count, page: page}));
      const json = await res.json();
      setState(json);
    })();
  };
  const handleChangePage = (event: any, newPage: any) => {
    reload(opts.numRows, newPage+1);
  };
  const handleChangeRowsPerPage = (event: any) => {
    const page = Math.floor(((state.page_info.page - 1) * opts.numRows) / +event.target.value);
    setOpts({
      ...opts,
      numRows: +event.target.value,
    });
    reload(+event.target.value, page+1);
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
    <InfoCard title='L1→L2 Transactions' sx={{height:''}}>
      <Table size='small'>
        <TableHead sx={{bgcolor:'background.default'}}>
          <TableRow>
            <TableCell>{t('Block Number')}</TableCell>
            <TableCell>{t('Queue Index')}</TableCell>
            <TableCell>{t('L2 Tx Hash')}</TableCell>
            <TableCell>
              <Link sx={{fontWeight:'inherit'}} component='button' underline='none' onClick={toggleTimestamp}>
                {opts.datetime ? t('Date Time (UTC)') : t('Age')}
              </Link>
            </TableCell>
            <TableCell>{t('L1 Tx Hash')}</TableCell>
            <TableCell>{t('L1 Tx Origin')}</TableCell>
            <TableCell>{t('Gas Limit')}</TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {
            state.records
            ? state.records.map((row, index) => (
              <TableRow key={index}>
                <TableCell>
                  <L1BlockLink blockNumber={row.l1_block_number} />
                </TableCell>
                <TableCell>
                  {row.queue_index}
                </TableCell>
                <TableCell>
                  <Box sx={{display:'flex',minWidth:'150px'}}>
                    <L2TransactionLink sx={{width:0,flexGrow:1,flexBasis:0}} hash={row.l2_tx_hash} />
                  </Box>
                </TableCell>
                <TableCell>
                  <Typography noWrap={true}>
                    {opts.datetime ? new Date(+row.timestamp * 1000).toLocaleString() : timeSince(row.timestamp)}
                  </Typography>
                </TableCell>
                <TableCell>
                  <Box sx={{display:'flex',minWidth:'150px'}}>
                    <L1TransactionLink sx={{width:0,flexGrow:1,flexBasis:0}} hash={row.l1_tx_hash} />
                  </Box>
                </TableCell>
                <TableCell>
                  <Box sx={{display:'flex',minWidth:'150px'}}>
                    <L1AddressLink sx={{width:0,flexGrow:1,flexBasis:0}} address={row.l1_tx_origin} />
                  </Box>
                </TableCell>
                <TableCell>{Number(row.gas_limit).toLocaleString()}</TableCell>
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
                page={state.page_info.page-1}
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
                sx={{borderBottom:'none'}}
              />
              : null
            }
          </TableRow>
        </TableFooter>
      </Table>
    </InfoCard>
  );
}

export default L1L2TransactionsList;