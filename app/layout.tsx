import NavBar from './components/NavBar'
import './globals.css'
import type { Metadata } from 'next'

export const metadata: Metadata = {
  title: 'Create Next App',
  description: 'Generated by create next app',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en" className='bg-gradient-to-b dark:from-red-950 dark:to-black from-red-200 to-white'>
      <body className='flex'>
        <NavBar />
        {children}
        </body>
    </html>
  )
}
