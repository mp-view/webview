import { USER_EVENT } from '../constants/USER_EVENT'

export function TrafficLight() {
  return (
    <div class="group flex space-x-6px">
      <button
        class="title-btn bg-#fa5348"
        onClick={() => window.ipc.postMessage(USER_EVENT.CLOSE)}
      >
        <div class="i-material-symbols:close op0 transition-opacity duration-75 group-hover:op100" />
      </button>
      <button
        class="title-btn bg-#fcb729"
        onClick={() => window.ipc.postMessage(USER_EVENT.MINIMIZE)}
      >
        <div class="i-material-symbols:remove op0 transition-opacity duration-75 group-hover:op100" />
      </button>
    </div>
  )
}
