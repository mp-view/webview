import type { PhoneInfoData } from './../constants/phoneInfo'

export function DeviceInfo({ info }: { info: PhoneInfoData }) {
  return (
    <div class="cursor-default ml8!">
      <div class="group flex cursor-pointer items-center c-#fff" id="device-change">
        <div>
          {info.name}
        </div>
        <div class="i-lucide-chevrons-up-down ml2 opacity-50"></div>
      </div>
      <div class="text-14px c-gray/80">
        {info.size[0]}
        x
        {info.size[1]}
      </div>
    </div>
  )
}
