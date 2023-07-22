'use client'

import { usePathname } from 'next/navigation'
import { useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
// import { AnimatePresence, motion } from 'framer-motion'
import RacePageHeader from '../../components/RacePageHeader'
import ListBlock from '../../components/ListBlock'
import RaceBlock from '../../components/RaceBlock'

interface YearlyRaceDataTypes {
    circuit_id: string,
    name: string,
    date: string,
    time: string,
    country_code: string,
}

export default function RacePageSlug() {
    const [race, setRace] = useState([]);
    const pathname = usePathname();
    const year = pathname.replace(/\D/g, '');

    useEffect(() => {
        invoke<any>('get_races', { year: year })
            .then((response) => {
                setRace(response);
            })
            .catch(console.error);
    }, []);

    return (
        <main className="min-h-screen max-w-5xl w-full mx-auto px-10 overflow-hidden justify-center">
            <RacePageHeader heading={year + ' Archive'} removeImg="true" />
            <ListBlock title={'All Events'}>
                {race && race.map((race: YearlyRaceDataTypes) =>
                    <RaceBlock key={race.circuit_id} circuitName={race.name} winner={'NULL'} date={race.date} time={race.time} flagcode={race.country_code} />
                )}
            </ListBlock>
        </main>
    )
}
