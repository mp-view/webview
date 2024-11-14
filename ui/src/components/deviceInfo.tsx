import type { PhoneInfoData } from './../constants/phoneInfo'
import {
  useClick,
  useDismiss,
  useFloating,
  useInteractions,
} from '@floating-ui/react'
import { useState } from 'preact/hooks'

import { phoneInfoData } from './../constants/phoneInfo'

export function DeviceInfo({ info }: { info: PhoneInfoData }) {
  const [isOpen, setIsOpen] = useState(false)

  const { refs, floatingStyles, context } = useFloating({
    open: isOpen,
    onOpenChange: setIsOpen,
    placement: 'bottom',
  })
  const click = useClick(context)
  const dismiss = useDismiss(context)

  const { getReferenceProps, getFloatingProps } = useInteractions([click, dismiss])

  return (
    <div class="cursor-default ml8!">
      <div
        class="group flex cursor-pointer items-center c-#fff"
        ref={refs.setReference}
        {...getReferenceProps()}
      >
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

      {
        isOpen && (
          <div
            ref={refs.setFloating}
            style={floatingStyles}
            {...getFloatingProps()}
          >
            {phoneInfoData.map(item => (
              <div class="rounded-md bg-white p4 shadow-md">
                <div class="flex items-center">
                  <div class="i-lucide-phone-call mr4"></div>
                  <div>{item.name}</div>
                </div>
              </div>
            ))}
          </div>
        )
      }
    </div>
  )
}
