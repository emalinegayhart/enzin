export interface PaginatorProps {
  current: number
  total: number
  limit: number
  onChange: (page: number) => void
}

export function Paginator({ current, total, limit, onChange }: PaginatorProps) {
  const totalPages = Math.ceil(total / limit)
  const currentPage = current + 1

  const isFirstPage = current === 0
  const isLastPage = current >= totalPages - 1

  return (
    <div>
      <button disabled={isFirstPage} onClick={() => onChange(current - 1)}>
        Previous
      </button>
      <span>Page {currentPage} of {totalPages}</span>
      <button disabled={isLastPage} onClick={() => onChange(current + 1)}>
        Next
      </button>
    </div>
  )
}
