import { renderHook, act, waitFor } from '@testing-library/react'
import { useEnzinSearch } from './useEnzinSearch'
import { describe, it, expect, beforeEach, vi } from 'vitest'

describe('useEnzinSearch', () => {
  beforeEach(() => { global.fetch = vi.fn() })

  it('should initialize with empty results', () => {
    const { result } = renderHook(() => useEnzinSearch({ apiUrl: 'http://localhost:7700', indexName: 'test' }))
    expect(result.current.results).toEqual([])
    expect(result.current.total).toBe(0)
    expect(result.current.loading).toBe(false)
    expect(result.current.error).toBeNull()
  })

  it('should perform a search', async () => {
    global.fetch = vi.fn(() => Promise.resolve({ ok: true, json: () => Promise.resolve({ hits: [{ id: '1', title: 'Test' }], total: 1 }) } as Response))
    const { result } = renderHook(() => useEnzinSearch({ apiUrl: 'http://localhost:7700', indexName: 'test' }))
    act(() => { result.current.search('test') })
    await waitFor(() => { expect(result.current.results).toHaveLength(1); expect(result.current.total).toBe(1) })
  })

  it('should handle search errors', async () => {
    global.fetch = vi.fn(() => Promise.resolve({ ok: false, statusText: 'Not Found' } as Response))
    const { result } = renderHook(() => useEnzinSearch({ apiUrl: 'http://localhost:7700', indexName: 'test' }))
    act(() => { result.current.search('test') })
    await waitFor(() => { expect(result.current.error).not.toBeNull() })
  })

  it('should toggle fuzzy search', () => {
    const { result } = renderHook(() => useEnzinSearch({ apiUrl: 'http://localhost:7700', indexName: 'test' }))
    expect(result.current.fuzzy).toBe(true)
    act(() => { result.current.setFuzzy(false) })
    expect(result.current.fuzzy).toBe(false)
  })

  it('should disable fuzzy when configured', () => {
    const { result } = renderHook(() => useEnzinSearch({ apiUrl: 'http://localhost:7700', indexName: 'test', fuzzy: false }))
    expect(result.current.fuzzy).toBe(false)
  })

  it('should update page', () => {
    const { result } = renderHook(() => useEnzinSearch({ apiUrl: 'http://localhost:7700', indexName: 'test', limit: 20 }))
    expect(result.current.page).toBe(0)
    act(() => { result.current.setPage(2) })
    expect(result.current.page).toBe(2)
  })
})
