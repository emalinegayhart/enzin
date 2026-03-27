import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { Paginator } from './Paginator'
import { describe, it, expect, vi } from 'vitest'

describe('Paginator', () => {
  it('should display current page', () => {
    render(<Paginator current={0} total={100} limit={20} onChange={() => {}} />)
    expect(screen.getByText('Page 1 of 5')).toBeInTheDocument()
  })

  it('should disable prev on first page', () => {
    render(<Paginator current={0} total={100} limit={20} onChange={() => {}} />)
    expect(screen.getByText('Previous')).toBeDisabled()
  })

  it('should disable next on last page', () => {
    render(<Paginator current={4} total={100} limit={20} onChange={() => {}} />)
    expect(screen.getByText('Next')).toBeDisabled()
  })

  it('should call onChange with next page', async () => {
    const onChange = vi.fn()
    render(<Paginator current={0} total={100} limit={20} onChange={onChange} />)
    await userEvent.click(screen.getByText('Next'))
    expect(onChange).toHaveBeenCalledWith(1)
  })

  it('should call onChange with prev page', async () => {
    const onChange = vi.fn()
    render(<Paginator current={2} total={100} limit={20} onChange={onChange} />)
    await userEvent.click(screen.getByText('Previous'))
    expect(onChange).toHaveBeenCalledWith(1)
  })

  it('should enable both buttons on middle page', () => {
    render(<Paginator current={2} total={100} limit={20} onChange={() => {}} />)
    expect(screen.getByText('Previous')).not.toBeDisabled()
    expect(screen.getByText('Next')).not.toBeDisabled()
  })
})
