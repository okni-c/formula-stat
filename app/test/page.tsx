'use client'

import Image from 'next/image'
import Link from 'next/link'
import { AnimatePresence, motion } from 'framer-motion'
import SendData from '../sendData'

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
        <h1 className='dark:text-white text-4xl font-bold drop-shadow-md'>Test Page</h1>
        <p className='my-5 dark:text-white'>This is the landing page.</p>
        <Link href='/' className='hover:scale-105 duration-100 rounded-lg bg-blue-600 px-4 py-2 text-white font-semibold'>Back to Home</Link>
        <SendData />
      </motion.main>
    </AnimatePresence>
  )
}
