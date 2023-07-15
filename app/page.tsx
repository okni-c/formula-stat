'use client'

import { AnimatePresence, motion } from 'framer-motion'

export default function Home() {
  return (
    <AnimatePresence>
      <motion.main className="flex min-h-screen w-full justify-center"
        initial={{ opacity: 0, scale: 0.5 }}
        animate={{ opacity: 1, scale: 1 }}
        exit={{ opacity: 0, scale: 0.5 }}
        transition={{
          duration: 0.1,
          delay: 0.1,
          ease: [0, 0.71, 0.2, 1.01]
        }}>
        <h1 className='text-3xl dark:text-white text-black'>Home Page</h1>
      </motion.main>
    </AnimatePresence>
  )
}
