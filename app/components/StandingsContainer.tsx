import { useEffect, useState } from "react";
import { invoke } from '@tauri-apps/api/tauri';

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

export default function StandingsContainer(functionName: string) {
    const [driver, setDriver] = useState([]);

    useEffect(() => {
      invoke<any>(functionName)
        .then((response) => {
          setDriver(response);
        })
        .catch(console.error);
    }, [functionName]);
    return (
        <section className="w-full h-full bg-gradient-to-b from-neutral-900 to-neutral-800 rounded-3xl flex justify-center p-5">
            <p className="bg-clip-text text-5xl text-transparent font-black bg-gradient-to-b from-white to-neutral-400">Driver Standings</p>
            {driver && driver.map((driver: DriverTypes) => 
                <p>{driver.surename}</p>
            )}
        </section>
    )
}