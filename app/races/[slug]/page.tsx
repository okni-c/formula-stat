'use client'

import { usePathname } from 'next/navigation'
import { useQuery } from '@tanstack/react-query';
import { fetchRaces } from '@/app/fetchers/fetchRaces';
import RacePageHeader from '../../components/RacePageHeader'
import ListBlock from '../../components/ListBlock'
import RaceBlock from '../../components/RaceBlock'
import { LoadingScreen } from '@/app/components/LoadingScreen';
import { YearlyRaceDataTypes } from '@/app/interfaces/interfaces';

export default function RacePageSlug() {

    const pathname = usePathname();
    const year = pathname.replace(/\D/g, '');

    const { data: race, isLoading, isError, isSuccess } = useQuery<any>({ queryKey: ['race', year], queryFn: () => fetchRaces(year) });

    if (isLoading) {
        return <LoadingScreen />;
    }

    if (isError) {
        return <div>Error occurred while fetching data.</div>;
    }
    if (isSuccess) {
        return (
            <main className="min-h-screen max-w-5xl w-full mx-auto px-10 overflow-hidden justify-center">
                <RacePageHeader heading={year + ' Archive'} removeImg="true" />
                <ListBlock title={'All Events'}>
                    {race.map((race: YearlyRaceDataTypes) =>
                        <RaceBlock key={race.date} circuitName={race.name} winner={'NULL'} date={race.date} time={race.time} flagcode={race.country_code} />
                    )}
                </ListBlock>
            </main>
        )
    }
}
