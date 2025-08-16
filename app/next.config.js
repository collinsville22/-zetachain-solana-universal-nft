/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  swcMinify: true,
  transpilePackages: ['@solana/wallet-adapter-base', '@solana/wallet-adapter-react'],
  webpack: (config, { isServer }) => {
    if (!isServer) {
      config.resolve.fallback = {
        ...config.resolve.fallback,
        crypto: require.resolve('crypto-browserify'),
        stream: require.resolve('stream-browserify'),
        url: require.resolve('url'),
        zlib: require.resolve('browserify-zlib'),
        https: require.resolve('https-browserify'),
        http: require.resolve('stream-http'),
        fs: false,
      };
    }
    return config;
  },
  env: {
    NEXT_PUBLIC_SOLANA_RPC_HOST: process.env.NEXT_PUBLIC_SOLANA_RPC_HOST || 'https://api.devnet.solana.com',
    NEXT_PUBLIC_PROGRAM_ID: process.env.NEXT_PUBLIC_PROGRAM_ID || 'EiGgwyFXtqcNEutPaUe94J9c9sPaPnDWj64sFcD7W9sz',
  },
  experimental: {
    esmExternals: 'loose',
  },
}

module.exports = nextConfig