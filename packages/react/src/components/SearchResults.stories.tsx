import type { Meta, StoryObj } from '@storybook/react'
import { SearchResults } from './SearchResults'
const meta = { title: 'Components/SearchResults', component: SearchResults, parameters: { layout: 'centered' } } satisfies Meta<typeof SearchResults>
export default meta
type Story = StoryObj<typeof meta>
export const WithResults: Story = { args: { results: [{ id: '1', title: 'Result' }], loading: false, error: null, total: 1, renderResult: (r) => <div>{r.title}</div> } }
export const Loading: Story = { args: { results: [], loading: true, error: null } }
export const NoResults: Story = { args: { results: [], loading: false, error: null, total: 0 } }
export const WithError: Story = { args: { results: [], loading: false, error: new Error('Failed'), total: 0 } }
