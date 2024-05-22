import withMarkdoc from '@markdoc/next.js';
import withSearch from './src/markdoc/search.mjs';

/** @type {import('next').NextConfig} */
const nextConfig = {
  pageExtensions: ['js', 'jsx', 'md', 'ts', 'tsx'],
};

const enhancedConfig = withMarkdoc({ schemaPath: './src/markdoc' })(nextConfig);

export default withSearch(enhancedConfig, {
  env: {
    NEXT_PUBLIC_API_HOST: process.env.API_HOST,
  },
});
