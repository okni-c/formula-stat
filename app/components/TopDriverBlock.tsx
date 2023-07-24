import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";
import Flag from "react-world-flags";

const cssObject: any = [{'1': 'dark:from-yellow-500 dark:to-black from-yellow-300 to-white'}, {'2': 'dark:from-zinc-500 dark:to-black from-zinc-400 to-white'}, {'3': 'dark:from-amber-800 dark:to-black from-amber-600 to-white'}];

interface TopDriverBlockProps {
    countryCode: string;
    forename: string;
    surename: string;
    points: string;
    position: string;
    driverId: string;
  }

export default function TopDriverBlock({ countryCode, forename, surename, points, position, driverId }: TopDriverBlockProps) {
    const [gradient, setGradient] = useState<string>('');
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
      }, [position]);

    return (
        <div className={'border dark:border-neutral-800 border-neutral-300 flex flex-col justify-center items-center w-52 bg-gradient-to-b rounded-2xl drop-shadow-sm cursor-pointer mb-4 ' + gradient} onClick={() => router.push('/driver/' + { driverId })}>
            <div className="relative w-full">
                <p className="absolute top-2 left-3 dark:text-white text-black font-bold drop-shadow-md">{position}</p>
                <p className="absolute top-2 right-3 dark:text-white text-black font-bold drop-shadow-md">{points} PTS</p>
                <div className="w-10 h-7 drop-shadow-md absolute top-9 right-3">
                    <Flag code={countryCode} className="rounded-[0.25rem] object-cover" />
                </div>
            </div>
            <img src="./max-v.png" alt="TopDriver photo" className="w-44" />
            <div className="dark:bg-black bg-white rounded-bl-2xl rounded-br-2xl w-full flex justify-center py-2">
                <p className="dark:text-white text-black font-black text-lg">{forename} {surename}</p>
            </div>
        </div>
    )
}