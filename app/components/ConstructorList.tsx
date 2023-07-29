import { useQuery } from '@tanstack/react-query';
import ConstructorBlock from "./ConstructorBlock";
import { fetchConstructorStandings } from "../fetchers/fetchConstructorStandings";
import { ConstructorTypes } from '../interfaces/interfaces';
import TopConstructorBlock from './TopConstructorBlock';

export default function ConstructorList() {
    const { data, isLoading, isError, isSuccess } = useQuery<any>({ queryKey: ['constructorStandings'], queryFn: fetchConstructorStandings });

    if (isLoading) {
        return <div>Loading...</div>;
    }

    if (isError) {
        return <div>Error occurred while fetching data.</div>;
    }

    const TopConstructorLoop = () => {
        if (data) {
            return (
                <div className="flex flex-row justify-center gap-5">
                    {data &&
                        data
                            .slice(1, 2) // Limit to the first 5 elements
                            .map((data: ConstructorTypes) => (
                                <TopConstructorBlock
                                    key={data.constructor_id}
                                    countryCode={data.country_code}
                                    name={data.name}
                                    points={data.points}
                                    position={data.position}
                                    constructorId={data.constructor_id}
                                    constructorRef={data.constructor_ref}
                                />
                            ))}
                    {data &&
                        data
                            .slice(0, 1) // Limit to the first 5 elements
                            .map((data: ConstructorTypes) => (
                                <TopConstructorBlock
                                    key={data.constructor_id}
                                    countryCode={data.country_code}
                                    name={data.name}
                                    points={data.points}
                                    position={data.position}
                                    constructorId={data.constructor_id}
                                    constructorRef={data.constructor_ref}
                                />
                            ))}
                    {data &&
                        data
                            .slice(2, 3) // Limit to the first 5 elements
                            .map((data: ConstructorTypes) => (
                                <TopConstructorBlock
                                    key={data.constructor_id}
                                    countryCode={data.country_code}
                                    name={data.name}
                                    points={data.points}
                                    position={data.position}
                                    constructorId={data.constructor_id}
                                    constructorRef={data.constructor_ref}
                                />
                            ))}
                </div>
            )
        }
    }

    const ConstructorLoop = () => {
        if (data) {
            return (
                <>
                    {data &&
                        data
                            .slice(3, 10) // Limit to the first 5 elements
                            .map((data: ConstructorTypes) => (
                                <ConstructorBlock
                                    key={data.constructor_id}
                                    countryCode={data.country_code}
                                    name={data.name}
                                    points={data.points}
                                    position={data.position}
                                    constructorId={data.constructor_id}
                                    constructorRef={data.constructor_ref}
                                />
                            ))}
                </>
            )
        }
    }

    if (isSuccess) {
        return (
            <>
                <TopConstructorLoop />
                <ConstructorLoop />
            </>
        )
    }
}