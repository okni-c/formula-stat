import { useRouter } from "next/navigation";
import { useState, useEffect } from "react";
import Flag from "react-world-flags";

const cssObject: any = [{ '1': 'dark:from-yellow-500 dark:to-black from-yellow-300 to-white' }, { '2': 'dark:from-zinc-500 dark:to-black from-zinc-400 to-white' }, { '3': 'dark:from-amber-800 dark:to-black from-amber-800 to-white' }];

export default function TopConstructorBlock({ countryCode, position, constructorId, name, points, constructorRef }: any) {
    const [gradient, setGradient] = useState<string>('');
    const [sufix, setSufix] = useState<string>('');
    const router = useRouter();

    useEffect(() => {
        // Check if position exists in cssObject array
        const cssEntry = cssObject.find((entry: any) => entry.hasOwnProperty(position));

        // If cssEntry exists, set the gradient state to its value
        if (cssEntry) {
            const key = Object.keys(cssEntry)[0];
            // setGradient(cssEntry[key]);
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
        <div className={'border dark:border-neutral-800 border-neutral-300 flex flex-col justify-between items-center w-52 bg-gradient-to-b rounded-2xl  cursor-pointer mb-4 pt-9 px-5' + gradient} onClick={() => router.push('/')}>
            <div className="relative w-full">
                <p className="absolute top-[-27px] left-[-5px] dark:text-white text-black  text-xl font-black flex items-center [text-shadow:_0_1px_3px_#e7e7e7a6]">{position}<span className="text-base font-semibold">{sufix}</span></p>
                <p className="absolute top-[-25px] right-[-5px] dark:text-white text-black font-extrabold  [text-shadow:_0_1px_3px_#e7e7e7a6]">{points} PTS</p>
                <div className="hidden w-10 h-7  absolute top-9 right-3">
                    <Flag code={countryCode} className="rounded-[0.25rem] object-cover" />
                </div>
            </div>
            <img src={'/constructors/' + constructorRef + '_logo.png'} alt="TopConstructor photo" className="h-24 object-contain" />
            <div className="bg-transparent rounded-bl-2xl rounded-br-2xl w-full flex justify-center pt-2 pb-3">
                <p className="dark:text-white text-black font-black text-lg">{name}</p>
            </div>
        </div>
    )
}