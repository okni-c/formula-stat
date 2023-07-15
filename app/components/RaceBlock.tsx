import Flag from "react-world-flags";

export default function RaceBLock({ location, winner, date, flagcode }: any) {
    return (
        <div className="flex gap-6 justify-between items-center dark:bg-black bg-zinc-200 bg-opacity-20 py-2 px-4 rounded-md my-1">
            <div className="flex items-center min-w-[160px]">
                <div className="w-10 h-7 drop-shadow-md flex mr-4"><Flag code={flagcode} className="rounded-sm object-cover" /></div>
                <p className="dark:text-white text-black">{location}</p>
            </div>
            <p className="dark:text-white text-black">🏆 {winner}</p>
            <p className="dark:text-white text-black">{date}</p>
        </div>
    );
}