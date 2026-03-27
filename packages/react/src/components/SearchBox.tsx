import { useState } from 'react'
import { useDebouncedCallback } from '../hooks'

export interface SearchBoxProps {
  onSearch: (query: string) => void
  placeholder?: string
  debounceMs?: number
}

export function SearchBox({
  onSearch,
  placeholder = 'Search...',
  debounceMs = 300
}: SearchBoxProps) {
  const [input, setInput] = useState('')
  const debouncedSearch = useDebouncedCallback(onSearch, debounceMs)

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const value = e.target.value
    setInput(value)
    debouncedSearch(value)
  }

  return (
    <input
      type="text"
      placeholder={placeholder}
      value={input}
      onChange={handleChange}
    />
  )
}
