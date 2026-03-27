import type { Meta, StoryObj } from '@storybook/react'
import { Search } from './Search'
const meta = { title: 'Components/Search', component: Search, parameters: { layout: 'centered' } } satisfies Meta<typeof Search>
export default meta
type Story = StoryObj<typeof meta>
export const Default: Story = { args: { apiUrl: 'http://localhost:7700', indexName: 'products' } }
export const FuzzyDisabled: Story = { args: { apiUrl: 'http://localhost:7700', indexName: 'products', fuzzy: false } }
export const CustomRenderer: Story = { args: { apiUrl: 'http://localhost:7700', indexName: 'products', renderResult: (r) => <div>{r.title}</div> } }
export const CustomLimit: Story = { args: { apiUrl: 'http://localhost:7700', indexName: 'products', limit: 10 } }
