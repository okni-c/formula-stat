import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";
import Flag from "react-world-flags";
import { TopDriverBlockTypes } from "../interfaces/interfaces";

const cssObject: any = [{ '1': 'dark:from-yellow-500 dark:to-black from-yellow-300 to-white' }, { '2': 'dark:from-zinc-500 dark:to-black from-zinc-400 to-white' }, { '3': 'dark:from-amber-800 dark:to-black from-amber-600 to-white' }];

export default function TopDriverBlock({ countryCode, forename, surename, points, position, driverId, driverRef }: TopDriverBlockTypes) {
    const [gradient, setGradient] = useState<string>('');
    const [sufix, setSufix] = useState<string>('');
    const router = useRouter();

    useEffect(() => {
        // Check if position exists in cssObject array
        const cssEntry = cssObject.find((entry: any) => entry.hasOwnProperty(position));

        // If cssEntry exists, set the gradient state to its value
        if (cssEntry) {
            const key = Object.keys(cssEntry)[0];
            setGradient(cssEntry[key]);
        } else {
            // If position is not found in cssObject, set a default value or handle accordingly
            setGradient('from-neutral-800 to-black');
        }

        switch (position) {
            case 1:
                setSufix('st');
                break;
            case 2:
                setSufix('nd')
                break;
            case 3:
                setSufix('rd')
                break;
        }
    }, [position]);

    return (
        <div className={'border dark:border-neutral-800 border-neutral-300 flex flex-col justify-center items-center w-52 bg-gradient-to-b rounded-2xl drop-shadow-sm cursor-pointer mb-4 ' + gradient} onClick={() => router.push('/')}>
            <div className="relative w-full">
                <p className="absolute top-2 left-3 dark:text-white text-black drop-shadow-md text-xl font-black flex items-center">{position}<span className="text-base font-semibold">{sufix}</span></p>
                <p className="absolute top-2 right-3 dark:text-white text-black font-extrabold drop-shadow-md">{points} PTS</p>
                <div className="w-10 h-7 drop-shadow-md absolute top-9 right-3">
                    <Flag code={countryCode} className="rounded-[0.25rem] object-cover" />
                </div>
            </div>
            <img src={'/drivers/' + driverRef + '_ds.png'} alt="TopDriver photo" className="w-44" />
            <div className="dark:bg-black bg-white rounded-bl-2xl rounded-br-2xl w-full flex justify-center py-2">
                <p className="dark:text-white text-black font-black text-lg">{forename} {surename}</p>
            </div>
        </div>
    )
}