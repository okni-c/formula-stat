import { useRouter } from "next/navigation";
import Flag from "react-world-flags";

export default function DriverBlock({ countryCode, forename, surename, points, position, driverId }: any) {
    const router = useRouter();

    return (
        <div className="flex gap-6 justify-between items-center bg-gradient-to-r hover:dark:bg-neutral-800 from-transparent dark:to-black to-neutral-100 hover:bg-neutral-300 cursor-pointer py-2 px-4 rounded-md my-1 drop-shadow-sm" onClick={() => router.push('/')}>
            <div className="flex gap-8 items-center justify-start">
                <div className="max-w-[8px] w-full">
                    <p className="dark:text-white text-black text-lg font-semibold">{position}</p>
                </div>
                <div className="w-10 h-7 drop-shadow-md flex justify-center">
                    <Flag code={countryCode} className="rounded-sm object-cover" />
                </div>
            </div>
            <div className="flex items-center min-w-[160px] justify-start">
                <p className="dark:text-white text-black">{forename} {surename}</p>
            </div>
            <div className="flex items-center min-w-[160px] justify-end">
                <p className="dark:text-white text-black">{points}</p>
            </div>

        </div>
    )
}