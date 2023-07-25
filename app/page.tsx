'use client'

import HomePageHeader from './components/HomePageHeader'
import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import NextEventBlock from './components/NextEventBlock';
import { AnimatePresence, motion } from 'framer-motion';

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
      <main className="min-h-screen max-w-5xl w-full mx-auto px-10 overflow-hidden justify-center">
        <AnimatePresence>
          <motion.div initial={{ x: -100 }}
                    animate={{ x: 0 }}
                    exit={{ x: -100 }}>
            <HomePageHeader circuitName={nextEvent.grand_prix_name} round={nextEvent.round} removeImg={false} />
            <NextEventBlock nextEvent={nextEvent} />
          </motion.div>
        </AnimatePresence>
      </main>
  )
}
