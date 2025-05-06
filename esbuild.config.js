const esbuild = require('esbuild');

esbuild.build({
  entryPoints: ['./esbuild_src/pages/TopPageAction.tsx'],
  bundle: true,
  outfile: './public_html/js/action/TopPageAction.js.react',
  minify: true,
  sourcemap: false,
  define: {
    'process.env.NODE_ENV': '"production"',
  },
  loader: {
    '.ts': 'ts',
    '.tsx': 'tsx',
  },
  target: ['es2020'],
  format: 'iife',
}).catch(() => process.exit(1));

