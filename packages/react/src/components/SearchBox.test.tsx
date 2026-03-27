import { render, screen, cleanup } from '@testing-library/react'
import { SearchBox } from './SearchBox'
import { describe, it, expect, afterEach } from 'vitest'

describe('SearchBox', () => {
  afterEach(() => cleanup())

  it('should render input with placeholder', () => {
    render(<SearchBox onSearch={() => {}} placeholder="Search products..." />)
    expect(screen.getByPlaceholderText('Search products...')).toBeInTheDocument()
  })

  it('should render with default placeholder', () => {
    render(<SearchBox onSearch={() => {}} />)
    expect(screen.getByPlaceholderText('Search...')).toBeInTheDocument()
  })

  it('should have correct input type', () => {
    const { container } = render(<SearchBox onSearch={() => {}} />)
    const input = container.querySelector('input')
    expect(input?.type).toBe('text')
  })

  it('should accept custom debounce delay', () => {
    render(<SearchBox onSearch={() => {}} debounceMs={500} />)
    expect(screen.getByPlaceholderText('Search...')).toBeInTheDocument()
  })
})
