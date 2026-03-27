import { useState, useCallback } from 'react'

export function usePagination(initialPage: number = 0) {
  const [page, setPage] = useState(initialPage)

  const goToPage = useCallback((newPage: number) => {
    setPage(Math.max(0, newPage))
  }, [])

  const nextPage = useCallback(() => {
    setPage((prev) => prev + 1)
  }, [])

  const prevPage = useCallback(() => {
    setPage((prev) => Math.max(0, prev - 1))
  }, [])

  return { page, goToPage, nextPage, prevPage }
}
