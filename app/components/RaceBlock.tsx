import Flag from "react-world-flags";
import { useRouter } from "next/navigation";

export default function RaceBLock({ location, winner, date, flagcode }: any) {
    const router = useRouter()

    return (
        <div className="flex gap-6 justify-between items-center hover:dark:bg-zinc-800 dark:bg-black bg-zinc-200 hover:bg-zinc-300 bg-opacity-20 cursor-pointer py-2 px-4 rounded-md my-1 drop-shadow-md" onClick={() => router.push('/races/'+{location})}>
            <div className="flex items-center min-w-[160px]">
                <div className="w-10 h-7 drop-shadow-md flex mr-4"><Flag code={flagcode} className="rounded-sm object-cover" /></div>
                <p className="dark:text-white text-black">{location}</p>
            </div>
            <p className="dark:text-white text-black">🏆 {winner}</p>
            <p className="dark:text-white text-black">{date}</p>
        </div>
    );
}