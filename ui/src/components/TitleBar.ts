// @unocss-include

import { phoneInfoData } from '../constants/phoneInfo'
import { USER_EVENT } from '../constants/USER_EVENT'
import { PhoneInfo } from './PhoneInfo'

/** 移除按钮监听 */
export function removeTitleBarListener(e: MouseEvent | TouchEvent) {
  if (e.target instanceof HTMLButtonElement)
    return true
  return false
}

export function TitleBar(element: HTMLDivElement) {
  element.innerHTML = /* html */`
  <div class="flex justify-between items-center pt3">
    <div class="space-x-6px ml3 flex items-center">
      <button class="title-btn bg-#fa5348" id="close-btn"></button>
      <button class="title-btn bg-#fcb729" id="minimize-btn"></button>
      <button class="title-btn bg-#2bc037" id="maximize-btn"></button>
      <div id="phone-info"></div>
    </div>
    <div class="mr3">
      <button class="icon-btn" id="pin-btn">
        <div class="i-carbon:pin">
      </button>
      <button class="icon-btn" id="devtools-btn">
        <div class="i-carbon:terminal">
      </button>
      <button class="icon-btn" id="menu-btn">
        <div class="i-carbon:list">
      </button>
    </div>
  </div>
  `

  // 事件映射
  const buttonEvents: { [key: string]: USER_EVENT } = {
    'close-btn': USER_EVENT.CLOSE,
    'minimize-btn': USER_EVENT.MINIMIZE,
    'maximize-btn': USER_EVENT.MAXIMIZE,
    'pin-btn': USER_EVENT.PIN,
    'devtools-btn': USER_EVENT.DEVTOOLS,
  }

  // 绑定按钮点击事件
  Object.keys(buttonEvents).forEach((id) => {
    const button = <HTMLButtonElement>element.querySelector(`#${id}`)
    button.addEventListener('click', () => window.ipc.postMessage(buttonEvents[id]))
  })

  element.addEventListener('mousedown', (e) => {
    if (removeTitleBarListener(e))
      return

    if (e.button === 0) {
      window.ipc.postMessage(USER_EVENT.DRAG_WINDOW)
    }
  })

  PhoneInfo(
    document.querySelector<HTMLDivElement>('#phone-info')!,
    phoneInfoData[0],
  )
}
