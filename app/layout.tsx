'use client'

import NavBar from './components/NavBar'
import YearSelector from './components/YearSelector'
import './globals.css'
import { usePathname } from 'next/navigation'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'

const queryClient = new QueryClient()

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  const pathname = usePathname();

  return (
    <html lang="en" className='bg-gradient-to-b dark:from-red-900 dark:to-black from-sky-200 to-white'>
      <QueryClientProvider client={queryClient}>
        <body>
          <NavBar />
          <YearSelector />
          <div className={pathname.startsWith('/races') ? 'ml-[16rem] flex' : 'ml-[9rem] flex'}>
            {children}
          </div>
        </body>
      </QueryClientProvider>
    </html>
  )
}
