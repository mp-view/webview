import { exec, spawn } from 'node:child_process'
import os from 'node:os'

let playProcess = null

function buildAndPlay() {
  const buildProcess = spawn('yarn', ['build:debug'], {
    stdio: 'inherit',
    shell: true,
  })

  buildProcess.on('close', (code) => {
    if (code === 0) {
      playProcess = spawn('yarn', ['play'], {
        stdio: 'inherit',
        shell: true,
      })
    }
  })
}

function killProcess(callback) {
  const platform = os.platform()
  const pid = playProcess.pid
  const command = platform === 'win32' ? `taskkill /PID ${pid} /T /F` : `pkill -P ${pid}`

  exec(command, (err) => {
    if (err) {
      console.error(`Failed to kill process ${pid}:`, err)
      throw new Error('Failed to kill process')
    }
    else {
      console.log(`Killed process ${pid}`)
      playProcess = null
      callback()
    }
  })
}

function run() {
  if (playProcess) {
    killProcess(
      () => {
        buildAndPlay()
      },
    )
  }
  else {
    buildAndPlay()
  }
}

export {
  run,
}
