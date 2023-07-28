'use client'

import HomePageHeader from './components/HomePageHeader';
import { useQuery } from '@tanstack/react-query'
import NextEventBlock from './components/NextEventBlock';
import { motion } from 'framer-motion';
import { fetchNextEvent } from './fetchers/fetchNextEvent';
import StandingsContainer from './components/StandingsContainer';
import DriverList from './components/DriverList';
import ConstructorList from './components/ConstructorList';
import { LoadingScreen } from './components/LoadingScreen';

export default function Home() {

  const { data: nextEvent, isLoading, isError, isSuccess } = useQuery<any>({ queryKey: ['nextEvent'], queryFn: fetchNextEvent });

  if (isLoading) {
    return <LoadingScreen />
  }

  if (isError) {
    return <div>Error occurred while fetching data.</div>;
  }

  if (isSuccess) {
    return (
      <motion.main className="min-h-screen max-w-5xl w-full mx-auto px-10 overflow-hidden justify-center no-scrollbar" initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
      transition={{ duration: 0.6 }}>
        <div>
          <HomePageHeader circuitName={nextEvent.grand_prix_name} round={nextEvent.round} removeImg={false} />
          <NextEventBlock nextEvent={nextEvent} />
          <StandingsContainer title={'Driver Standings'}>
            <DriverList />
          </StandingsContainer>
          <StandingsContainer title={'Constructor Standings'}>
            <ConstructorList />
          </StandingsContainer>
        </div>
      </motion.main>
    );
  }
}

