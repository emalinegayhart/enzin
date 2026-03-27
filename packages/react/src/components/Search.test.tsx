import { render, screen, waitFor } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { Search } from './Search'
import { describe, it, expect, vi, beforeEach } from 'vitest'

describe('Search', () => {
  beforeEach(() => {
    global.fetch = vi.fn(() =>
      Promise.resolve({
        ok: true,
        json: () => Promise.resolve({ hits: [], total: 0 })
      } as Response)
    )
  })

  it('should render search box', () => {
    render(<Search apiUrl="http://localhost:7700" indexName="test" />)
    expect(screen.getByRole('textbox')).toBeInTheDocument()
  })

  it('should render with custom renderer', async () => {
    global.fetch = vi.fn(() =>
      Promise.resolve({
        ok: true,
        json: () => Promise.resolve({ hits: [{ id: '1', title: 'Result' }], total: 1 })
      } as Response)
    )
    render(
      <Search
        apiUrl="http://localhost:7700"
        indexName="test"
        renderResult={(r) => <div>{r.title}</div>}
      />
    )
    await userEvent.type(screen.getByRole('textbox'), 'test')
    await waitFor(() => expect(screen.getByText('Result')).toBeInTheDocument())
  })

  it('should hide paginator when total < limit', async () => {
    global.fetch = vi.fn(() =>
      Promise.resolve({
        ok: true,
        json: () => Promise.resolve({ hits: [{ id: '1' }], total: 5 })
      } as Response)
    )
    render(<Search apiUrl="http://localhost:7700" indexName="test" limit={20} />)
    await userEvent.type(screen.getByRole('textbox'), 'test')
    await waitFor(() => {
      expect(screen.queryAllByRole('button')).toHaveLength(0)
    })
  })

  it('should show paginator when total > limit', async () => {
    global.fetch = vi.fn(() =>
      Promise.resolve({
        ok: true,
        json: () => Promise.resolve({ hits: Array(20).fill({ id: '1' }), total: 50 })
      } as Response)
    )
    render(<Search apiUrl="http://localhost:7700" indexName="test" limit={20} />)
    await userEvent.type(screen.getByRole('textbox'), 'test')
    await waitFor(() => expect(screen.getByText(/Page 1/)).toBeInTheDocument())
  })
})
