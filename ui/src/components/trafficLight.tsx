import { USER_EVENT } from '../constants/USER_EVENT'

export function TrafficLight() {
  return (
    <>
      <button class="title-btn bg-#fa5348" onClick={() => window.ipc.postMessage(USER_EVENT.CLOSE)}></button>
      <button class="title-btn bg-#fcb729" onClick={() => window.ipc.postMessage(USER_EVENT.MINIMIZE)}></button>
      <button class="title-btn bg-#2bc037" onClick={() => window.ipc.postMessage(USER_EVENT.MAXIMIZE)}></button>
    </>
  )
}
