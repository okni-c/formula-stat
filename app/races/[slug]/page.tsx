'use client'

import { usePathname } from 'next/navigation'
import { useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { AnimatePresence, motion } from 'framer-motion'
import RacePageHeader from '../../components/RacePageHeader'
import ListBlock from '../../components/ListBlock'
import RaceBlock from '../../components/RaceBlock'

export default function RacePageSlug() {
    const [data, setData] = useState([]);
    const pathname = usePathname();
    const year = pathname.replace(/\D/g, '');

    useEffect(() => {
        invoke<any>('get_races', { year: year })
          .then((response) => {
            setData(response);
          })
          .catch(console.error);
      }, []);
      console.log(data)

      interface YearlyRaceDataTypes {
        circuit_id: string,
        name: string,
        date: string,
      }
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
                <RacePageHeader heading={year + ' Archive'} removeImg="true" />
                <ListBlock title={'All Events'}>
                    {data && data.map((data:YearlyRaceDataTypes) => 
                    <RaceBlock key={data.circuit_id} location={data.name} winner={'NULL'} date={data.date} flagcode={276} />
                    )}
                </ListBlock>
            </motion.main>
        </AnimatePresence>
    )
}
