import React, {useEffect} from 'react';
import InfoCard from '../../../components/InfoCard';
import {Table, TableBody, TableCell, TableFooter, TableHead, TablePagination, TableRow,} from '@mui/material';
import {useRecoilState} from 'recoil';
import {options, state as _state} from './state';
import {useTranslation} from 'react-i18next';
import {api} from '../../../utils/urlResolver';
import {ValidatorLink} from '../../../components/Link';

function ValidatorList() {
  const {t} = useTranslation('', {useSuspense: false});
  const [state, setState] = useRecoilState(_state);
  const [opts, setOpts] = useRecoilState(options);

  const reload = (count: number, page: number) => {
    (async () => {
      const res = await fetch(api('/validator', undefined, {count: count, page: page}));
      console.log(res);
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

  console.log(state.records);

  return (
    <InfoCard title='Validator' sx={{height: ''}}>
      <Table size='small'>
        <TableHead sx={{bgcolor: 'background.default'}}>
          <TableRow>
            <TableCell>{t('Validator')}</TableCell>
            <TableCell>{t('Operator Address')}</TableCell>
            <TableCell>{t('Voting Power')}</TableCell>
            <TableCell>{t('Commission')}</TableCell>
            <TableCell>{t('Jailed')}</TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {
            state.records
              ? state.records.map((row, index) => (
                <TableRow key={index}>
                  <TableCell>{row.description.moniker}</TableCell>
                  <TableCell>
                    <ValidatorLink address={row.operator_address}/>
                  </TableCell>
                  <TableCell>{parseInt(row.tokens).toLocaleString()}</TableCell>
                  <TableCell>{`${(parseFloat(row.commission.commission_rates.rate) * 100).toFixed(2)}%`}</TableCell>
                  <TableCell>{row.jailed.toString()}</TableCell>
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

export default ValidatorList;
