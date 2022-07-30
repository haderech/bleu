import React from 'react';
import {useRecoilState} from 'recoil';
import {Box, Button, Divider, Drawer, IconButton, Popover,} from '@mui/material';
import KeyboardArrowDownIcon from '@mui/icons-material/KeyboardArrowDown';
import MenuIcon from '@mui/icons-material/Menu';
import Logo from './Header/Logo';
import ContentBody from './ContentBody';
import {options} from './Header/state';
import MenuItem from './Header/MenuItem';

const menuButton: Readonly<any> = {
  color: 'text.primary',
  textTransform: 'none',
  fontWeight: 'normal',
  padding: '8px 16px 8px 20px',
  fontSize: '0.9rem',
};

const menuButtonActive = {
  ...menuButton,
  color: 'text.secondary',
};

const menuPopup = {
  padding: '12px 0px 16px 0px',
};

function Header(props: any) {
  const [opts, setOpts] = useRecoilState(options);
  const headerEl = React.useRef<HTMLDivElement | null>(null);

  const openMenu = (index: number) => {
    return (event: any) => {
      setOpts({
        ...opts,
        index: index,
        anchorEl: event.currentTarget,
        anchorCoords: event.currentTarget.getBoundingClientRect(),
      });
    };
  };
  const closeMenu = () => {
    setOpts({
      index: -1,
      anchorEl: null,
      anchorCoords: null,
    });
  };
  const outMenu = (event: any) => {
    if (opts.index >= 0 && opts.anchorCoords) {
      if (event.clientX < opts.anchorCoords.left || event.clientX >= opts.anchorCoords.right ||
        event.clientY < opts.anchorCoords.top || event.clientY >= opts.anchorCoords.bottom) {
        closeMenu();
      }
    } else {
      closeMenu();
    }
  };
  const onMenu = (event: any) => {
    event.stopPropagation();
  };

  return (
    <ContentBody sx={{bgcolor: 'background.paper', zIndex: {xs: 1301, sm: 1301, md: 1100}}} content={{py: '8px'}}
                 ref={headerEl}>
      <Box sx={{display: 'flex', justifyContent: 'space-between', alignItems: 'center'}}>
        <Logo/>
        <Box sx={{display: {xs: 'none', sm: 'none', md: 'flex'}}}>
          <Button href='/'>Home</Button>
          <Button
            sx={opts.index === 0 ? menuButtonActive : menuButton}
            size='small'
            href='#'
            onMouseEnter={openMenu(0)}
            endIcon={<KeyboardArrowDownIcon sx={{marginLeft: '-6px'}}/>}
          >Blockchain</Button>
          {/*
          <Button
            sx={opts.index === 1 ? menuButtonActive : menuButton}
            size='small'
            href='#'
            onMouseEnter={openMenu(1)}
            endIcon={<KeyboardArrowDownIcon sx={{marginLeft: '-6px'}} />}
          >Tokens</Button>*/}
          <Popover open={opts.index === 0} anchorEl={opts.anchorEl}
                   anchorOrigin={{horizontal: 'left', vertical: 'bottom'}} onMouseMove={outMenu}
                   sx={{cursor: 'pointer'}}>
            <Box sx={menuPopup} onMouseMove={onMenu} onMouseLeave={outMenu}>
              <MenuItem href='/blocks'>Blocks</MenuItem>
              <MenuItem href='/txs'>Transactions</MenuItem>
              <Divider sx={{my:'8px'}} />
              <MenuItem href='/validators'>Validators</MenuItem>
              <MenuItem href='/proposals'>Proposals</MenuItem>
            </Box>
          </Popover>
          <Popover open={opts.index === 1} anchorEl={opts.anchorEl}
                   anchorOrigin={{horizontal: 'left', vertical: 'bottom'}} onMouseMove={outMenu}
                   sx={{cursor: 'pointer'}}>
            <Box sx={menuPopup} onMouseMove={onMenu} onMouseLeave={outMenu}>
            </Box>
          </Popover>
        </Box>

        <IconButton edge='start' color='inherit' aria-label='menu' sx={{display: {sm: 'block', md: 'none'}}}
                    onClick={openMenu(99)}>
          <MenuIcon/>
        </IconButton>
      </Box>
      <Drawer open={opts.index === 99} onClose={closeMenu} anchor='top'
              PaperProps={{sx: {position: 'absolute', top: headerEl?.current?.offsetHeight}}}>
        <Box>
          <MenuItem sx={{width: '100%-56px', height: '40px'}} href='/txs'>Transactions</MenuItem>
          <MenuItem sx={{width: '100%-56px', height: '40px'}} href='/blocks'>Transaction Batches</MenuItem>
          <MenuItem sx={{width: '100%-56px', height: '40px'}} href='/blocks?isState=true'>State Batches</MenuItem>
        </Box>
      </Drawer>
    </ContentBody>
  );
}

export default Header;
