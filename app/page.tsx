'use client'

import { AnimatePresence, motion } from 'framer-motion'
import HomePageHeader from './components/HomePageHeader'

export default function Home() {
  return (
    <AnimatePresence>
      <motion.main className="min-h-screen max-w-5xl w-full mx-auto px-10 overflow-hidden justify-center"
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        exit={{ opacity: 0 }}
        transition={{
          duration: 0.1,
          delay: 0.1
        }}>
        <HomePageHeader round={10} circuitName={'Hungarian Gran Prix'} removeImg={false} />
      </motion.main>
    </AnimatePresence>
  )
}
