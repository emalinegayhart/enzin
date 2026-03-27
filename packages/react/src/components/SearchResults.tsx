import { SearchResult } from '../types'

export interface SearchResultsProps {
  results: SearchResult[]
  loading?: boolean
  error?: Error | null
  total?: number
  renderResult?: (result: SearchResult) => React.ReactNode
}

export function SearchResults({
  results,
  loading = false,
  error = null,
  total,
  renderResult
}: SearchResultsProps) {
  if (loading) return <div>Loading...</div>
  if (error) return <div>Error: {error.message}</div>
  if (results.length === 0) return <div>No results found</div>

  return (
    <div>
      {total !== undefined && <div>Found {total} results</div>}
      <div>
        {results.map((result, idx) => (
          <div key={idx}>
            {renderResult ? renderResult(result) : <pre>{JSON.stringify(result)}</pre>}
          </div>
        ))}
      </div>
    </div>
  )
}
