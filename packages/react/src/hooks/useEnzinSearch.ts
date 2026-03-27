import { useState, useCallback } from 'react'
import { SearchResult, UseEnzinSearchOptions, UseEnzinSearchReturn } from '../types'

export function useEnzinSearch({
  apiUrl,
  indexName,
  limit = 20,
  fuzzy: fuzzyEnabled = true
}: UseEnzinSearchOptions): UseEnzinSearchReturn {
  const [results, setResults] = useState<SearchResult[]>([])
  const [total, setTotal] = useState(0)
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<Error | null>(null)
  const [fuzzy, setFuzzy] = useState(fuzzyEnabled)
  const [offset, setOffset] = useState(0)
  const [page, setPage] = useState(0)

  const search = useCallback(
    async (query: string) => {
      if (!query.trim()) {
        setResults([])
        setTotal(0)
        return
      }

      setLoading(true)
      setError(null)

      try {
        const params = new URLSearchParams({
          q: query,
          fuzzy: fuzzy.toString(),
          limit: limit.toString(),
          offset: offset.toString()
        })

        const response = await fetch(
          `${apiUrl}/indexes/${indexName}/search?${params}`,
          { method: 'GET' }
        )

        if (!response.ok) {
          throw new Error(`Search failed: ${response.statusText}`)
        }

        const data = await response.json()
        setResults(data.hits || [])
        setTotal(data.total || 0)
      } catch (err) {
        setError(err instanceof Error ? err : new Error('Unknown error'))
        setResults([])
        setTotal(0)
      } finally {
        setLoading(false)
      }
    },
    [apiUrl, indexName, fuzzy, limit, offset]
  )

  return {
    results,
    total,
    loading,
    error,
    search,
    fuzzy,
    setFuzzy,
    limit,
    offset,
    setPage,
    page
  }
}
