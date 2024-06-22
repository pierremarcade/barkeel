/** @type {import('next').NextConfig} */
const nextConfig = {
  pageExtensions: ['js', 'jsx', 'md', 'ts', 'tsx'],
};

export default {
  env: {
    NEXT_PUBLIC_API_HOST: process.env.API_HOST,
  },
};
