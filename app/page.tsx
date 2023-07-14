import Image from 'next/image'
import Link from 'next/link'

export default function Home() {
  return (
    <main className="flex min-h-screen flex-col items-center p-24">
      <h1 className='text-4xl font-bold'>Hello World</h1>
      <p className='my-5'>Test</p>
      <Link href='/test'>Go to Test Page</Link>
    </main>
  )
}
