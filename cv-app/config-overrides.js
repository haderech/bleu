/* config-overrides.js */
/* eslint-disable react-hooks/rules-of-hooks */
const { addBabelPlugin, override } = require('customize-cra');

module.exports = override(addBabelPlugin(['babel-plugin-direct-import', {
  modules: ['@mui/material', '@mui/icons-material'],
}]));