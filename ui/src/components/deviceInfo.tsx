import type { PhoneInfoData } from './../constants/phoneInfo'
import { USER_EVENT } from '~/constants/USER_EVENT'
import { phoneInfoData } from './../constants/phoneInfo'
import Popover from './ui/popover'

export function DeviceInfo({ info }: { info: PhoneInfoData }) {
  const content = (
    <div class="flex flex-col px-2 py-1.5">
      {phoneInfoData.map(item => (
        <div key={item.name} class="cursor-pointer rounded-md p1.5 hover:bg-zinc-8">
          <div class="flex items-center gap-x-2" onClick={() => window.ipc.postMessage(`${USER_EVENT.DEVICE_INFO}-${JSON.stringify(item)}`)}>
            <div class="text-14px">{item.name}</div>
          </div>
        </div>
      ))}
    </div>
  )

  return (
    <div class="cursor-default ml8!">
      <Popover
        content={content}
        trigger="click"
        onOpenChange={() => window.ipc.postMessage(USER_EVENT.MENU_MAXIMIZE)}
      >
        <div class="group flex cursor-pointer items-center c-#fff">
          <div>
            {info.name}
          </div>
          <div class="i-lucide-chevrons-up-down ml2 opacity-50"></div>
        </div>
      </Popover>
      <div class="text-14px c-gray/80">
        {info.size[0]}
        x
        {info.size[1]}
      </div>
    </div>
  )
}
