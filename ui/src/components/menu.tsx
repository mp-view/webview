import { USER_EVENT } from '../constants/USER_EVENT'

export function Menu() {
  return (
    <div class="mr3">
      <button class="icon-btn" onClick={() => window.ipc.postMessage(USER_EVENT.PIN)}>
        <div class="i-carbon:pin" />
      </button>
      <button class="icon-btn" onClick={() => window.ipc.postMessage(USER_EVENT.DEVTOOLS)}>
        <div class="i-carbon:terminal" />
      </button>
      <button class="icon-btn" onClick={() => window.ipc.postMessage(USER_EVENT.SCREENSHOT)}>
        <div class="i-carbon:camera" />
      </button>
      <button class="icon-btn">
        <div class="i-carbon:list" />
      </button>
    </div>
  )
}
