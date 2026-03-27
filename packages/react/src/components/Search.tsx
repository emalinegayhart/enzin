import { useEffect } from 'react'
import { SearchResult } from '../types'
import { useEnzinSearch } from '../hooks'
import { SearchBox } from './SearchBox'
import { SearchResults } from './SearchResults'
import { Paginator } from './Paginator'

export interface SearchProps {
  apiUrl: string
  indexName: string
  limit?: number
  fuzzy?: boolean
  renderResult?: (result: SearchResult) => React.ReactNode
}

export function Search({
  apiUrl,
  indexName,
  limit = 20,
  fuzzy = true,
  renderResult
}: SearchProps) {
  const { results, total, loading, error, search, page, setPage } =
    useEnzinSearch({ apiUrl, indexName, limit, fuzzy })

  useEffect(() => {
    const offset = page * limit
  }, [page, limit])

  const showPaginator = total > limit

  return (
    <div>
      <SearchBox
        onSearch={search}
      />
      <SearchResults
        results={results}
        loading={loading}
        error={error}
        total={total}
        renderResult={renderResult}
      />
      {showPaginator && (
        <Paginator current={page} total={total} limit={limit} onChange={setPage} />
      )}
    </div>
  )
}
