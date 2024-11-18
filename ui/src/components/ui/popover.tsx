import type { PropsWithChildren, ReactNode } from 'react'
import {
  flip,
  offset,
  useClick,
  useDismiss,
  useFloating,
  useHover,
  useInteractions,
} from '@floating-ui/react'
import { createPortal } from 'preact/compat'
import { useEffect, useMemo, useState } from 'preact/hooks'

type Alignment = 'start' | 'end'
type Side = 'top' | 'right' | 'bottom' | 'left'
type AlignedPlacement = `${Side}-${Alignment}`

interface PopoverProps extends PropsWithChildren {
  content: ReactNode
  trigger?: 'hover' | 'click'
  placement?: Side | AlignedPlacement
  open?: boolean
  onOpenChange?: (open: boolean) => void
}

export default function Popover(props: PopoverProps) {
  const {
    open,
    onOpenChange,
    content,
    children,
    trigger = 'hover',
    placement = 'bottom',
  } = props

  const [isOpen, setIsOpen] = useState(open)
  const [referenceWidth, setReferenceWidth] = useState(0)

  const { refs, floatingStyles, context } = useFloating({
    open: isOpen,
    onOpenChange: (open) => {
      onOpenChange?.(open)
      setIsOpen(open)
    },
    placement,
    middleware: [
      offset(2),
      flip(),
    ],
  })

  const interaction = trigger === 'hover' ? useHover(context) : useClick(context)

  const dismiss = useDismiss(context)

  const { getReferenceProps, getFloatingProps } = useInteractions([
    interaction,
    dismiss,
  ])

  const el = useMemo(() => {
    const el = document.createElement('div')
    el.className = 'wrapper'

    document.body.appendChild(el)
    return el
  }, [])

  useEffect(() => {
    if (refs.reference.current) {
      setReferenceWidth((refs.reference.current as HTMLSpanElement).offsetWidth)
    }
  }, [refs.reference.current])

  const floating = isOpen && (
    <div
      class="border-1 border-zinc-8 rounded-md border-solid bg-black c-white backdrop-blur-sm"
      ref={refs.setFloating}
      style={{ ...floatingStyles, width: referenceWidth + 10 }}
      {...getFloatingProps()}
      onClick={() => setIsOpen(false)}
    >
      {content}
    </div>
  )

  return (
    <>
      <span
        ref={refs.setReference}
        {...getReferenceProps()}
      >
        {children}
      </span>
      {
        createPortal(floating, el)
      }
    </>
  )
}
