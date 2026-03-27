import type { Meta, StoryObj } from '@storybook/react'
import { Paginator } from './Paginator'
const meta = { title: 'Components/Paginator', component: Paginator, parameters: { layout: 'centered' } } satisfies Meta<typeof Paginator>
export default meta
type Story = StoryObj<typeof meta>
export const FirstPage: Story = { args: { current: 0, total: 100, limit: 20, onChange: (p) => console.log(p) } }
export const MiddlePage: Story = { args: { current: 2, total: 100, limit: 20, onChange: (p) => console.log(p) } }
export const LastPage: Story = { args: { current: 4, total: 100, limit: 20, onChange: (p) => console.log(p) } }
export const SinglePage: Story = { args: { current: 0, total: 15, limit: 20, onChange: (p) => console.log(p) } }
