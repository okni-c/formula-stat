'use client'

import HomePageHeader from './components/HomePageHeader';
import { useQuery } from '@tanstack/react-query'
import NextEventBlock from './components/NextEventBlock';
import { AnimatePresence, motion } from 'framer-motion';
import { fetchNextEvent } from './fetchers/fetchNextEvent';

export default function Home() {

  const { data: nextEvent, isLoading, isError } = useQuery<any>({ queryKey: ['nextEvent'], queryFn: fetchNextEvent });

  if (isLoading) {
    return <div>Loading...</div>;
  }

  if (isError) {
    return <div>Error occurred while fetching data.</div>;
  }

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
  );
}

