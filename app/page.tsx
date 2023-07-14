'use client'

import Image from 'next/image'
import Link from 'next/link'
import Greet from './greet'
import GetData from './getData'
import { AnimatePresence, motion } from 'framer-motion'

export default function Home() {
  return (
    <AnimatePresence>
      <motion.main className="flex min-h-screen flex-col items-center p-24"
        initial={{ opacity: 0, scale: 0.5 }}
        animate={{ opacity: 1, scale: 1 }}
        exit={{ opacity: 0, scale: 0.5 }}
        transition={{
          duration: 0.1,
          delay: 0.1,
          ease: [0, 0.71, 0.2, 1.01]
        }}>
        <h1 className='text-4xl font-bold drop-shadow-md'>Welcome to our App.</h1>
        <p className='my-5'>This is the landing page.</p>
        <Link href='/test' className='hover:scale-105 duration-50 rounded-lg bg-gradient-to-b from-blue-600 to-blue-800 px-4 py-2 text-white font-semibold shadow-md'>Go to Test Page</Link>
        <div className='my-8 flex flex-col gap-4 items-center'>
          <Greet />
          <GetData />
        </div>
      </motion.main>
    </AnimatePresence>
  )
}
