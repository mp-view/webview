import type { PhoneInfoData } from './../constants/phoneInfo'
import { phoneInfoData } from './../constants/phoneInfo'
import Popover from './ui/popover'

export function DeviceInfo({ info }: { info: PhoneInfoData }) {
  const content = (
    <div class="flex flex-col gap-y-4">
      {phoneInfoData.map(item => (
        <div key={item.name}>
          <div class="flex items-center gap-x-2">
            <div class="i-lucide-info-circle-fill text-14px text-gray/60"></div>
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
