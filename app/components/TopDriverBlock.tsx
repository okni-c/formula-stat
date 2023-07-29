import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";
import Flag from "react-world-flags";
import { TopDriverBlockTypes } from "../interfaces/interfaces";

export default function TopDriverBlock({ countryCode, forename, surename, points, position, driverId, driverRef }: TopDriverBlockTypes) {
    const [sufix, setSufix] = useState<string>('');
    const router = useRouter();

    useEffect(() => {

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
        <div className={' dark:border-neutral-800 border-neutral-300 flex flex-col justify-center items-center w-52 rounded-2xl hover:shadow-[0_0_24px_0_rgba(14,14,14,0.2)] duration-150 ease-linear cursor-pointer mb-4'} onClick={() => router.push('/')}>
            <div className="relative w-full">
                <p className="absolute top-2 left-3 dark:text-white text-black drop-shadow-md text-xl font-black flex items-center [text-shadow:_0_1px_3px_#e7e7e7a6]">{position}<span className="text-base font-semibold">{sufix}</span></p>
                <p className="absolute top-2 right-3 dark:text-white text-black font-extrabold drop-shadow-md [text-shadow:_0_1px_3px_#e7e7e7a6]">{points} PTS</p>
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