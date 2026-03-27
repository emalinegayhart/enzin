import type { Meta, StoryObj } from '@storybook/react'
import { SearchBox } from './SearchBox'
const meta = { title: 'Components/SearchBox', component: SearchBox, parameters: { layout: 'centered' } } satisfies Meta<typeof SearchBox>
export default meta
type Story = StoryObj<typeof meta>
export const Default: Story = { args: { onSearch: (q) => console.log(q), placeholder: 'Search...' } }
export const CustomPlaceholder: Story = { args: { onSearch: (q) => console.log(q), placeholder: 'Find anything...' } }
export const CustomDebounce: Story = { args: { onSearch: (q) => console.log(q), debounceMs: 500 } }
