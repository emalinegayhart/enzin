import { renderHook, act } from '@testing-library/react'
import { usePagination } from './usePagination'
import { describe, it, expect } from 'vitest'

describe('usePagination', () => {
  it('should initialize with default page', () => {
    const { result } = renderHook(() => usePagination())
    expect(result.current.page).toBe(0)
  })

  it('should initialize with provided page', () => {
    const { result } = renderHook(() => usePagination(5))
    expect(result.current.page).toBe(5)
  })

  it('should go to next page', () => {
    const { result } = renderHook(() => usePagination())
    act(() => { result.current.nextPage() })
    expect(result.current.page).toBe(1)
  })

  it('should go to previous page', () => {
    const { result } = renderHook(() => usePagination(2))
    act(() => { result.current.prevPage() })
    expect(result.current.page).toBe(1)
  })

  it('should not go below page 0', () => {
    const { result } = renderHook(() => usePagination())
    act(() => { result.current.prevPage() })
    expect(result.current.page).toBe(0)
  })

  it('should go to specific page', () => {
    const { result } = renderHook(() => usePagination())
    act(() => { result.current.goToPage(5) })
    expect(result.current.page).toBe(5)
  })
})
