import React, {useEffect} from 'react';
import {useRecoilState} from 'recoil';
import {useTranslation} from 'react-i18next';
import {useLocation} from 'react-router-dom';
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
  Tooltip,
  Typography,
} from '@mui/material';
import InfoCard from '../../../components/InfoCard';
import {L2AddressLink, L2TransactionLink} from '../../../components/Link';
import {timeSince} from '../../../utils/time';
import {options, state as _state} from './state';
import {api} from '../../../utils/urlResolver';
import {toEther, txFee} from '../../../utils/ethUtils';

function TransactionsList() {
  const {t} = useTranslation('', {useSuspense: false});
  const [state, setState] = useRecoilState(_state);
  const [opts, setOpts] = useRecoilState(options);

  const {search} = useLocation();
  const searchParams = new URLSearchParams(search);
  const blockNum = searchParams.get('blockNum');
  const isState = searchParams.get('isState') === 'true';

  const reload = (count: number, page: number) => {
    if (!blockNum) {
      (async () => {
        const res = await fetch(api('/tx', undefined, {count: count, page: page}));
        const json = await res.json();
        setState(json);
      })();
    } else {
      if (!isState) {
        (async () => {
          const res = await fetch(api('/tx/tx-batch/index', blockNum, {count: count, page: page}));
          const json = await res.json();
          setState(json);
        })();
      } else {
        (async () => {
          const res = await fetch(api('/tx/stateroot-batch/index', blockNum, {count: count, page: page}));
          const json = await res.json();
          setState(json);
        })();
      }
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
    if (!blockNum) {
      return 'Transactions';
    } else {
      if (!isState) {
        return 'Transactions in Batch';
      } else {
        return 'Transactions in State Batch';
      }
    }
  };

  return (
    <InfoCard title={getTitle()} subtitle={blockNum ? `#${blockNum}` : null} sx={{height:''}}>
      <Table size='small'>
        <TableHead sx={{bgcolor:'background.default'}}>
          <TableRow>
            <TableCell>{t('Txn Hash')}</TableCell>
            <TableCell>{t('Method')}</TableCell>
            <TableCell>{t('Index')}</TableCell>
            <TableCell>
              <Link sx={{fontWeight:'inherit'}} component='button' underline='none' onClick={toggleTimestamp}>
                {opts.datetime ? t('Date Time (UTC)') : t('Age')}
              </Link>
            </TableCell>
            <TableCell>{t('From')}</TableCell>
            <TableCell>{t('To')}</TableCell>
            <TableCell>{t('Sequencer')}</TableCell>
            <TableCell>{t('Value')}</TableCell>
            <TableCell>{t('Tx Fee')}</TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {
            state.records
            ? state.records.map((row, index) => (
              <TableRow key={index}>
                <TableCell>
                  <Box sx={{display:'flex',minWidth:'150px'}}>
                    <L2TransactionLink sx={{width:0,flexGrow:1,flexBasis:0}} hash={row.tx.hash} />
                  </Box>
                </TableCell>
                <TableCell>
                  {row.tx.tx_input ? row.tx.tx_input.slice(0, 10) : null}
                </TableCell>
                <TableCell>
                  <L2TransactionLink hash={row.tx.index} />
                </TableCell>
                <TableCell>
                  <Typography noWrap={true}>
                    {opts.datetime ? new Date(+row.tx.l1_timestamp * 1000).toLocaleString() : timeSince(row.tx.l1_timestamp)}
                  </Typography>
                </TableCell>
                <TableCell>
                  <Box sx={{display:'flex',minWidth:'150px'}}>
                    <L2AddressLink sx={{width:0,flexGrow:1,flexBasis:0}} address={row.tx.from_address} />
                  </Box>
                </TableCell>
                <TableCell>
                  <Box sx={{display:'flex',minWidth:'150px'}}>
                    {
                      row.tx.to_address
                      ? <L2AddressLink sx={{width: 0, flexGrow: 1, flexBasis: 0}} address={row.tx.to_address}/>
                      : <Tooltip title={row.contract_address}>
                          <Link underline='none' href={`/account/${row.contract_address}`}>
                            Contract Creation
                          </Link>
                        </Tooltip>
                    }
                  </Box>
                </TableCell>
                <TableCell sx={{textTransform:'capitalize'}}>{row.tx.queue_origin}</TableCell>
                <TableCell>{toEther(row.tx.value)} Ether</TableCell>
                <TableCell><Typography variant='body2'>{txFee(row.gas_used, row.tx.gas_price)}</Typography></TableCell>
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

export default TransactionsList;