import React from 'react';
import {IconButton, InputBase, Paper,} from '@mui/material';
import SearchIcon from '@mui/icons-material/Search';
import {useHistory} from 'react-router-dom';

let keyword = '';

function setKeyword(input: string) {
  keyword = input;
}

function search() {
  if (keyword.length === 64) {
    return `/tx/hash/${keyword}`;
  } else {
    const height = parseInt(keyword);
    if (height) {
      return `/block/height/${height}`;
    }
  }
  return '';
}

function CustomizedInputBase() {
  const history = useHistory();

  return (
    <Paper
      component='form'
      sx={{
        p: '2px 4px',
        display: 'flex',
        alignItems: 'center',
        width: '100%',
        maxWidth: 650,
        marginLeft: '15px',
        marginRight: '15px'
      }}
      onSubmit={(e: any) => {
        e.preventDefault();
        const redirect = search();
        if (redirect.length > 0) {
          history.push(redirect);
          keyword = '';
        }
      }}
    >
      <InputBase
        sx={{ml: 1, flex: 1}}
        placeholder='Search ...'
        inputProps={{'aria-label': 'search'}}
        onChange={(e) => setKeyword(e.target.value)}
      />
      <IconButton type='submit' sx={{p: '10px'}} aria-label='search'>
        <SearchIcon/>
      </IconButton>
    </Paper>
  );
}

function SearchBar() {
  return (
    <CustomizedInputBase/>
  );
}

export default SearchBar;
