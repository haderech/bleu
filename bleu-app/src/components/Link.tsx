import React from 'react';
import {
  Link,
  Tooltip,
} from '@mui/material';

export function AddressLink(props: any) {
  return (
    <Tooltip title={props.address || ''} disableInteractive>
      <Link variant='mono' underline='none' noWrap={true} href={`/account/${props.address}`} sx={props.sx}>
        {props.address}
      </Link>
    </Tooltip>
  );
}

export function BlockTxnLink(props: any) {
  return (
    <Tooltip title={props.hash || ''} disableInteractive>
      <Link variant='mono' underline='none' noWrap={true} href={`/txs?blockNumber=${props.blockNumber}`} sx={props.sx}>
        {props.txn}
      </Link>
    </Tooltip>
  );
}

export function TransactionLink(props: any) {
  return (
    <Tooltip title={props.hash || ''} disableInteractive>
      <Link variant='mono' underline='none' noWrap={true} href={`/tx/${props.hash}`} sx={props.sx}>
        {props.hash}
      </Link>
    </Tooltip>
  );
}

export function BlockLink(props: any) {
  return (
    <Link variant='mono' underline='none' noWrap={true} href={`/block/${props.param}`} sx={props.sx}>
      {props.param}
    </Link>
  );
}
