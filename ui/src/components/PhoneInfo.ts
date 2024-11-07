// @unocss-include
import type { PhoneInfoData } from './../constants/phoneInfo'

export function PhoneInfo(
  element: HTMLDivElement,
  info: PhoneInfoData,
) {
  element.innerHTML = /* html */`
  <div class="ml8 cursor-default">
    <div class="c-#fff">${info.name}</div>
    <div class="c-gray/80 text-14px">${info.size[0]} x ${info.size[1]}</div>
  </div>
  `
}
