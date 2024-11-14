import { resolve } from 'node:path'
import preact from '@preact/preset-vite'
import UnoCSS from 'unocss/vite'
import { defineConfig } from 'vite'
import { viteSingleFile } from 'vite-plugin-singlefile'
import { run } from '../scripts/buildAndPlay.mjs'

export default defineConfig({
  resolve: {
    alias: {
      '~': resolve(__dirname, './src'),
    },
  },
  plugins: [
    UnoCSS(),
    preact(),
    viteSingleFile(),
    {
      name: 'rs-build-plugin',
      writeBundle() {
        run()
      },
    },
  ],
  build: {
    outDir: 'src',
    emptyOutDir: false,
    rollupOptions: {
      input: {
        main: resolve(__dirname, 'index.html'),
      },
    },
  },
})
