export interface SearchResult {
  [key: string]: string | number | boolean | null
}

export interface UseEnzinSearchOptions {
  apiUrl: string
  indexName: string
  limit?: number
  fuzzy?: boolean
}

export interface UseEnzinSearchReturn {
  results: SearchResult[]
  total: number
  loading: boolean
  error: Error | null
  search: (query: string) => Promise<void>
  fuzzy: boolean
  setFuzzy: (bool: boolean) => void
  limit: number
  offset: number
  setPage: (page: number) => void
  page: number
}
