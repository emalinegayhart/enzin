import { renderHook, act } from '@testing-library/react'
import { useDebounce } from './useDebounce'
import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest'

describe('useDebounce', () => {
  beforeEach(() => vi.useFakeTimers())
  afterEach(() => { vi.runOnlyPendingTimers(); vi.useRealTimers() })

  it('should return initial value immediately', () => {
    const { result } = renderHook(() => useDebounce('test', 300))
    expect(result.current).toBe('test')
  })

  it('should debounce value changes', () => {
    const { result, rerender } = renderHook(({ value }: { value: string }) => useDebounce(value, 300), { initialProps: { value: 'a' } })
    expect(result.current).toBe('a')
    rerender({ value: 'ab' })
    expect(result.current).toBe('a')
    act(() => vi.advanceTimersByTime(300))
    expect(result.current).toBe('ab')
  })

  it('should reset debounce on rapid changes', () => {
    const { result, rerender } = renderHook(({ value }: { value: string }) => useDebounce(value, 300), { initialProps: { value: 'a' } })
    rerender({ value: 'ab' })
    act(() => vi.advanceTimersByTime(100))
    rerender({ value: 'abc' })
    act(() => vi.advanceTimersByTime(100))
    expect(result.current).toBe('a')
    act(() => vi.advanceTimersByTime(300))
    expect(result.current).toBe('abc')
  })
})
