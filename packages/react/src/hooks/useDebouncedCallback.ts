import { useCallback, useRef } from 'react'

export function useDebouncedCallback<T extends (...args: any[]) => any>(
  callback: T,
  delayMs: number
): T {
  const timeoutRef = useRef<NodeJS.Timeout>()

  return useCallback(
    ((...args) => {
      clearTimeout(timeoutRef.current)
      timeoutRef.current = setTimeout(() => {
        callback(...args)
      }, delayMs)
    }) as T,
    [callback, delayMs]
  )
}
