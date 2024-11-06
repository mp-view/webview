import type { ChildProcess } from 'node:child_process'
import { exec, spawn } from 'node:child_process'
import os from 'node:os'
import { resolve } from 'node:path'
import UnoCSS from 'unocss/vite'
import { defineConfig } from 'vite'
import { viteSingleFile } from 'vite-plugin-singlefile'

let playProcess: ChildProcess | null = null

function buildAndPlay() {
  const buildProcess = spawn('yarn', ['build:debug'], {
    stdio: 'inherit',
    shell: true,
  })

  buildProcess.on('close', () => {
    playProcess = spawn('yarn', ['play'], {
      stdio: 'inherit',
      shell: true,
    })
  })
}

function killProcess(pid: number, callback: () => void) {
  const platform = os.platform()
  const command = platform === 'win32' ? `taskkill /PID ${pid} /T /F` : `kill -9 ${pid}`

  exec(command, (err) => {
    if (err) {
      console.error(`Failed to kill process ${pid}:`, err)
      throw new Error('Failed to kill process')
    }
    else {
      console.log(`Killed process ${pid}`)
      callback()
    }
  })
}

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
        if (playProcess) {
          killProcess(playProcess.pid!, () => {
            playProcess = null
            buildAndPlay()
          })
        }
        else {
          buildAndPlay()
        }
      },
    },
  ],
})
