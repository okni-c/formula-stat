'use client'

import { useQuery } from '@tanstack/react-query';
import { fetchRaces } from '@/app/fetchers/fetchRaces';
import RacePageHeader from '../components/RacePageHeader'
import ListBlock from '../components/ListBlock'
import RaceBlock from '../components/RaceBlock'
import { LoadingScreen } from '@/app/components/LoadingScreen';
import { YearlyRaceDataTypes } from '../interfaces/interfaces';

export default function RacePage() {
  const { data: race, isLoading, isError, isSuccess } = useQuery<any>({ queryKey: ['race', '2023'], queryFn: () => fetchRaces('2023') });

  if (isLoading) {
    return <LoadingScreen />
  }

  if (isError) {
    return <div>Error occurred while fetching data.</div>;
  }
  if (isSuccess) {
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
}
