import { useEffect, useState } from "react";
import { invoke } from '@tauri-apps/api/tauri';
import DriverBlock from "./DriverBlock";
import ConstructorBlock from "./ConstructorBlock";

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

interface ConstructorTypes {
    constructor_id: string,
    points: string,
    position: string,
    wins: string,
    name: string,
    nationality: string,
    country_code: string,
}

interface InputTypes {
    functionName: string,
    title: string,
}

export default function StandingsContainer({ functionName, title }: InputTypes) {
    const [data, setData] = useState([]);
    const [decider, setDecider] = useState('');

    useEffect(() => {
        invoke<any>(functionName)
            .then((response) => {
                setData(response);
            })
            .catch(console.error);

        setDecider(functionName);
    }, [functionName]);

    const DriverLoop = () =>
        // data && data.map((data: DriverTypes) =>
        //     <DriverBlock key={data.driver_id} countryCode={data.country_code} forename={data.forename} surename={data.surename} points={data.points} position={data.position} driverId={data.driver_id} />
        // )
        data &&
        data
            .slice(0, 5) // Limit to the first 5 elements
            .map((data: DriverTypes) =>
            <DriverBlock key={data.driver_id} countryCode={data.country_code} forename={data.forename} surename={data.surename} points={data.points} position={data.position} driverId={data.driver_id} />
        )

    const ConstructorLoop = () =>
        // data && data.map((data: ConstructorTypes) =>
        //     <ConstructorBlock key={data.constructor_id} countryCode={data.country_code} name={data.name} points={data.points} position={data.position} driverId={data.constructor_id} />
        // )
        data &&
        data
            .slice(0, 5) // Limit to the first 5 elements
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
        <section className="w-full bg-gradient-to-b dark:from-neutral-900 dark:to-black from-white to-transparent rounded-3xl flex flex-col px-5 py-4 my-10">
            <p className="bg-clip-text text-3xl text-transparent font-black bg-gradient-to-b dark:from-white dark:to-neutral-400 from-black to-neutral-700 pb-4 pl-3">{title}</p>
            {decider === 'get_home_page_driver_standings' ?
                <DriverLoop /> : <ConstructorLoop />
            }
        </section>
    )
}