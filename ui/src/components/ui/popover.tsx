import type { CSSProperties, PropsWithChildren, ReactNode } from 'react'
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
import { useMemo, useState } from 'preact/hooks'

type Alignment = 'start' | 'end'
type Side = 'top' | 'right' | 'bottom' | 'left'
type AlignedPlacement = `${Side}-${Alignment}`

interface PopoverProps extends PropsWithChildren {
  content: ReactNode
  trigger?: 'hover' | 'click'
  placement?: Side | AlignedPlacement
  open?: boolean
  onOpenChange?: (open: boolean) => void
  className?: string
  style?: CSSProperties
}

export default function Popover(props: PopoverProps) {
  const {
    open,
    onOpenChange,
    content,
    children,
    trigger = 'hover',
    placement = 'bottom',
    className,
    style,
  } = props

  const [isOpen, setIsOpen] = useState(open)

  const { refs, floatingStyles, context } = useFloating({
    open: isOpen,
    onOpenChange: (open) => {
      onOpenChange?.(open)
      setIsOpen(open)
    },
    placement,
    middleware: [
      offset(10),
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

  const floating = isOpen && (
    <div
      className="popover-floating"
      ref={refs.setFloating}
      style={floatingStyles}
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
        className={className}
        style={style}
      >
        {children}
      </span>
      {
        createPortal(floating, el)
      }
    </>
  )
}
