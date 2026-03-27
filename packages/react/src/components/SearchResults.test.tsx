import { render, screen } from '@testing-library/react'
import { SearchResults } from './SearchResults'
import { describe, it, expect } from 'vitest'

describe('SearchResults', () => {
  it('should show loading state', () => {
    render(<SearchResults results={[]} loading={true} error={null} />)
    expect(screen.getByText('Loading...')).toBeInTheDocument()
  })

  it('should show error state', () => {
    const error = new Error('Failed')
    render(<SearchResults results={[]} loading={false} error={error} />)
    expect(screen.getByText(/Error/)).toBeInTheDocument()
  })

  it('should show no results', () => {
    render(<SearchResults results={[]} loading={false} error={null} />)
    expect(screen.getByText('No results found')).toBeInTheDocument()
  })

  it('should render results', () => {
    const results = [{ id: '1', title: 'Test' }]
    render(<SearchResults results={results} loading={false} error={null} renderResult={(r) => <div>{r.title}</div>} />)
    expect(screen.getByText('Test')).toBeInTheDocument()
  })

  it('should display total count', () => {
    const results = [{ id: '1', title: 'Test' }]
    render(<SearchResults results={results} loading={false} error={null} total={42} />)
    expect(screen.getByText('Found 42 results')).toBeInTheDocument()
  })

  it('should render JSON by default', () => {
    const results = [{ id: '1', title: 'Test' }]
    render(<SearchResults results={results} loading={false} error={null} />)
    expect(screen.getByText(/id.*title/)).toBeInTheDocument()
  })
})
