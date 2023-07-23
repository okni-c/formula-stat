'use client'

import NavBar from './components/NavBar'
import YearSelector from './components/YearSelector'
import './globals.css'
import { usePathname } from 'next/navigation'

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  const pathname = usePathname();
  return (
    <html lang="en">
      <body className='bg-gradient-to-b dark:from-red-950 dark:to-black from-sky-200 to-white'>
        <NavBar />
        <YearSelector />
        <div className={pathname.startsWith('/races') ? 'ml-[16rem] flex' : 'ml-[9rem] flex'}>
          {children}
        </div>
      </body>
    </html>
  )
}
