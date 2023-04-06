import React, { useEffect } from 'react';
import {
  Box,
  Link,
  Table,
  TableBody,
  TableCell, TableFooter,
  TableHead, TablePagination,
  TableRow, Typography,
} from '@mui/material';
import { useRecoilState, useRecoilValue } from 'recoil';
import { options as rootOptions } from './state';
import { options, state as _state } from './Transactions/state';
import { AddressLink, BlockLink, TransactionLink } from '../../../components/Link';
import { timeSince } from '../../../utils/time';
import { toEther, txFee } from '../../../utils/ethUtils';
import { useTranslation } from 'react-i18next';
import { api } from '../../../utils/urlResolver';

function Transactions(props: any) {
  const { t } = useTranslation('', { useSuspense: false });
  const [state, setState] = useRecoilState(_state);
  const [opts, setOpts] = useRecoilState(options);
  const ropts = useRecoilValue(rootOptions);

  const reload = (address: string, count: number, page: number) => {
    if (address.length > 0) {
      (async () => {
        const res = await fetch(api('/txs', undefined, { count, page, address }));
        const json = await res.json();
        setState(json);
      })();
    }
  }
  const handleChangePage = (event: any, newPage: any) => {
    reload(ropts.address, opts.numRows, newPage + 1);
  };
  const handleChangeRowsPerPage = (event: any) => {
    const page = Math.floor(((state.page_info.page - 1) * opts.numRows) / +event.target.value);
    setOpts({
      ...opts,
      numRows: +event.target.value,
    });
    reload(ropts.address, +event.target.value, page + 1);
  };
  const toggleTimestamp = () => {
    setOpts({
      ...opts,
      datetime: !opts.datetime,
    });
  };

  useEffect(() => {
    reload(props.address, opts.numRows, 1);
  }, []);

  return (
    <React.Fragment>
      <Table size='small'>
        <TableHead sx={{ bgcolor: 'background.default' }}>
          <TableRow>
            <TableCell>
              Transaction Hash
            </TableCell>
            <TableCell>
              Method
            </TableCell>
            <TableCell>
              Block
            </TableCell>
            <TableCell>
              <Link sx={{ fontWeight: 'inherit' }} component='button' underline='none' onClick={toggleTimestamp}>
                {opts.datetime ? t('Date Time (UTC)') : t('Age')}
              </Link>
            </TableCell>
            <TableCell>
              From
            </TableCell>
            <TableCell>
              To
            </TableCell>
            <TableCell>
              Value
            </TableCell>
            <TableCell>
              Transaction Fee
            </TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {
            state.records
              ? state.records.map((row, index) => (
                <TableRow key={index}>
                  <TableCell>
                    <Box sx={{ display: 'flex', minWidth: '150px' }}>
                      <TransactionLink sx={{ width: 0, flexGrow: 1, flexBasis: 0 }} hash={row.hash} />
                    </Box>
                  </TableCell>
                  <TableCell>
                    <Typography variant='mono'>{row.tx_input ? row.tx_input.slice(0, 10) : null}</Typography>
                  </TableCell>
                  <TableCell>
                    <BlockLink blockNumber={row.block_number} />
                  </TableCell>
                  <TableCell>
                    <Typography noWrap={true}>
                      {opts.datetime ? new Date(+row.block_timestamp * 1000).toLocaleString() : timeSince(row.block_timestamp)}
                    </Typography>
                  </TableCell>
                  <TableCell>
                    <Box sx={{ display: 'flex', minWidth: '150px' }}>
                      {
                        ropts.address !== row.tx_from
                          ? <AddressLink sx={{ width: 0, flexGrow: 1, flexBasis: 0 }} address={row.tx_from} />
                          : <Typography variant='mono' sx={{ width: 0, flexGrow: 1, flexBasis: 0 }} noWrap={true}>{row.tx_from}</Typography>
                      }
                    </Box>
                  </TableCell>
                  <TableCell>
                    <Box sx={{ display: 'flex', minWidth: '150px' }}>
                      {
                        row.tx_to
                          ? ropts.address !== row.tx_to
                            ? <AddressLink sx={{ width: 0, flexGrow: 1, flexBasis: 0 }} address={row.tx_to} />
                            : <Typography variant='mono' sx={{ width: 0, flexGrow: 1, flexBasis: 0 }} noWrap={true}>{row.tx_to}</Typography>
                          : <Link underline='none' href={`/account/${row.contract_address}`}>Contract Creation</Link>
                      }
                    </Box>
                  </TableCell>
                  <TableCell>{toEther(row.tx_value)} Unit</TableCell>
                  <TableCell><Typography variant='body2'>{txFee(row.gas_used, row.gas_price)}</Typography></TableCell>
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
                  sx={{ borderBottom: 'none' }}
                />
                : null
            }
          </TableRow>
        </TableFooter>
      </Table>
    </React.Fragment>
  );
}

export default Transactions;