'use client'

import { ThemeProvider } from 'next-themes'
import {
  useQuery,
  useMutation,
  useQueryClient,
  QueryClient,
  QueryClientProvider,
} from '@tanstack/react-query'

const queryClient = new QueryClient()

export function Providers({ children }: { children: React.ReactNode }) {
  return (
    <ThemeProvider attribute="class" disableTransitionOnChange>
      <QueryClientProvider client={queryClient}>
      {children}
      </QueryClientProvider>
      
    </ThemeProvider>
  )
}
