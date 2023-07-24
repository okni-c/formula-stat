import { fetchDriverStandings } from "../fetchers/fetchDriverStandings";
import { useQuery } from '@tanstack/react-query';
import DriverBlock from "./DriverBlock";
import TopDriverBlock from "./TopDriverBlock";

interface DriverTypes {
    driver_id: string,
    points: string,
    position: string,
    wins: string,
    forename: string,
    surename: string,
    nationality: string,
    country_code: string,
}

export default function DriverList() {
    const { data, isLoading, isError } = useQuery<any>({ queryKey: ['driverStandings'], queryFn: fetchDriverStandings });

    if (isLoading) {
        return <div>Loading...</div>;
    }

    if (isError) {
        return <div>Error occurred while fetching data.</div>;
    }

    const TopDriverLoop = () => {
        return (
            <div className="flex flex-row justify-center gap-5">
                {data && data
                    .slice(0, 3)
                    .map((data: DriverTypes) =>
                        <TopDriverBlock key={data.driver_id} countryCode={data.country_code} forename={data.forename} surename={data.surename} points={data.points} position={data.position} driverId={data.driver_id} />
                    )}
            </div>
        )
    }

    const DriverLoop = () =>
        data &&
        data
            .slice(3, 10)
            .map((data: DriverTypes) =>
                <DriverBlock key={data.driver_id} countryCode={data.country_code} forename={data.forename} surename={data.surename} points={data.points} position={data.position} driverId={data.driver_id} />
            )

    return (
        <>
            <TopDriverLoop />
            <DriverLoop />
        </>
    )
}