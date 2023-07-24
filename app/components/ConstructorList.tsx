import { fetchDriverStandings } from "../fetchers/fetchDriverStandings";
import { useQuery } from '@tanstack/react-query';
import ConstructorBlock from "./ConstructorBlock";
import { fetchConstructorStandings } from "../fetchers/fetchConstructorStandings";

interface ConstructorTypes {
    constructor_id: string,
    points: string,
    position: string,
    wins: string,
    name: string,
    nationality: string,
    country_code: string,
}

export default function ConstructorList() {
    const { data, isLoading, isError } = useQuery<any>({ queryKey: ['constructorStandings'], queryFn: fetchConstructorStandings });

    if (isLoading) {
        return <div>Loading...</div>;
    }

    if (isError) {
        return <div>Error occurred while fetching data.</div>;
    }

    const ConstructorLoop = () =>
        data &&
        data
            .slice(0, 10) // Limit to the first 5 elements
            .map((data: ConstructorTypes) => (
                <ConstructorBlock
                    key={data.constructor_id}
                    countryCode={data.country_code}
                    name={data.name}
                    points={data.points}
                    position={data.position}
                    driverId={data.constructor_id}
                />
            ));

    return (
        <ConstructorLoop />
    )
}