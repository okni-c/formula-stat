'use client'

// import { AnimatePresence, motion } from 'framer-motion'
import { useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import RacePageHeader from '../components/RacePageHeader'
import ListBlock from '../components/ListBlock'
import RaceBlock from '../components/RaceBlock'

interface YearlyRaceDataTypes {
  circuit_id: string,
  name: string,
  date: string,
  time: string,
  country_code: string,
}

export default function RacePage() {
  const [race, setRace] = useState([]);
  useEffect(() => {
    invoke<any>('get_races', { year: '2023' })
      .then((response) => {
        setRace(response);
      })
      .catch(console.error);
  }, []);
  return (
    <main className="min-h-screen max-w-5xl w-full mx-auto px-10 overflow-hidden justify-center">
      <RacePageHeader heading={'2023 Archive'} removeImg="true" />
      <ListBlock title={'All Events'}>
        {race && race.map((race: YearlyRaceDataTypes) =>
          <RaceBlock key={race.circuit_id} circuitName={race.name} winner={'NULL'} date={race.date} time={race.time} flagcode={race.country_code} />
        )}
      </ListBlock>
    </main>
  )
}
