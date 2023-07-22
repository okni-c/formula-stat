import Flag from "react-world-flags";
import { useRouter } from "next/navigation";
import formatDate from "../hooks/formatDate";

interface RaceBlockTypes {
    circuitName: string,
    winner: string,
    date: string,
    flagcode: string,
    time: string,
}

export default function RaceBLock({ circuitName, winner, date, flagcode, time }: RaceBlockTypes) {
    const router = useRouter()

    return (
        <div className="flex gap-6 justify-between items-center bg-gradient-to-r hover:dark:bg-neutral-800 from-transparent dark:to-black to-neutral-100 hover:bg-neutral-300 bg-opacity-20 cursor-pointer py-2 px-4 rounded-md my-1 drop-shadow-sm" onClick={() => router.push('/races/' + { circuitName })}>
            <div className="flex items-center min-w-[160px]">
                <div className="w-10 h-7 drop-shadow-md flex mr-4 justify-center"><Flag code={flagcode} className="rounded-sm object-cover" /></div>
                <p className="dark:text-white text-black">{circuitName}</p>
            </div>
            <p className="dark:text-white text-black">🏆 {winner}</p>
            <p className="dark:text-white text-black">{formatDate(date, time)}</p>
        </div>
    );
}