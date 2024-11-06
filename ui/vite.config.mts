import { resolve } from 'node:path'
import UnoCSS from 'unocss/vite'
import { defineConfig } from 'vite'
import { viteSingleFile } from 'vite-plugin-singlefile'
import { run } from './../scripts/buildAndPlay.mjs'

export default defineConfig({
  build: {
    outDir: 'src',
    emptyOutDir: false,
    rollupOptions: {
      input: {
        main: resolve(__dirname, 'index.html'),
      },
    },
  },
  plugins: [
    UnoCSS(),
    viteSingleFile(),
    {
      name: 'rs-build-plugin',
      writeBundle() {
        run()
      },
    },
  ],
})
