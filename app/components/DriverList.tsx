import { fetchDriverStandings } from "../fetchers/fetchDriverStandings";
import { useQuery } from '@tanstack/react-query';
import DriverBlock from "./DriverBlock";

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

    const DriverLoop = () =>
        data &&
        data
            .slice(0, 5) // Limit to the first 5 elements
            .map((data: DriverTypes) =>
                <DriverBlock key={data.driver_id} countryCode={data.country_code} forename={data.forename} surename={data.surename} points={data.points} position={data.position} driverId={data.driver_id} />
            )

    return (
        <DriverLoop />
    )
}