'use client'

import { AnimatePresence, motion } from 'framer-motion'
import HomePageHeader from './components/HomePageHeader'
import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import HomePageRolex from './components/HomePageRolex';

export default function Home() {
  const [nextEvent, setNextEvent] = useState<any>({});

  useEffect(() => {
    invoke<any>('get_home_page_next_event')
      .then((response) => {
        setNextEvent(response);
      })
      .catch(console.error);
  }, []);

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
        <HomePageHeader circuitName={nextEvent.name} round={nextEvent.round} removeImg={false} />

        <HomePageRolex nextEvent={nextEvent} />

      </motion.main>
    </AnimatePresence>
  )
}
