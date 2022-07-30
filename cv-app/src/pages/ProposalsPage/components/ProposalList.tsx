import React, {useEffect} from 'react';
import InfoCard from '../../../components/InfoCard';
import {Box, Table, TableBody, TableCell, TableFooter, TableHead, TablePagination, TableRow,} from '@mui/material';
import {useRecoilState} from 'recoil';
import {options, state as _state} from './state';
import {useTranslation} from 'react-i18next';
import {api} from '../../../utils/urlResolver';
import {ProposalLink} from '../../../components/Link';
import {getSimpleStatus} from '../../../utils/message';

function ProposalList() {
  const {t} = useTranslation('', {useSuspense: false});
  const [state, setState] = useRecoilState(_state);
  const [opts, setOpts] = useRecoilState(options);

  const reload = (count: number, page: number) => {
    (async () => {
      const res = await fetch(api('/proposal', undefined, {count: count, page: page}));
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

  return (
    <InfoCard title='Proposal' sx={{height: ''}}>
      <Table size='small'>
        <TableHead sx={{bgcolor: 'background.default'}}>
          <TableRow>
            <TableCell>{t('Id')}</TableCell>
            <TableCell>{t('Title')}</TableCell>
            <TableCell>{t('Status')}</TableCell>
            <TableCell>{t('Voting Start')}</TableCell>
            <TableCell>{t('Submit Time')}</TableCell>
            <TableCell>{t('Total Deposit')}</TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {
            state.records
              ? state.records.map((row, index) => (
                <TableRow key={index}>
                  <TableCell>{row.proposal_id}</TableCell>
                  <TableCell>
                    <Box sx={{display: 'flex', minWidth: '300px'}}>
                      <ProposalLink sx={{width: 0, flexGrow: 1, flexBasis: 0}} title={row.content.title}
                                    id={row.proposal_id}/>
                    </Box>
                  </TableCell>
                  <TableCell>{getSimpleStatus(row.status)}</TableCell>
                  <TableCell>{row.voting_start_time}</TableCell>
                  <TableCell>{row.submit_time}</TableCell>
                  <TableCell>{row.total_deposit.length > 0 ? `${row.total_deposit[0]['amount']} ${row.total_deposit[0]['denom']}` : '0 uatom'}</TableCell>
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

export default ProposalList;
