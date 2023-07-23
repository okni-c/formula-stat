import { useRouter } from "next/navigation";
import Flag from "react-world-flags";

export default function ConstructorBlock({ countryCode, position, constructorId, name, points }: any) {
    const router = useRouter();

    return (
        <div className="flex gap-6 justify-between items-center bg-gradient-to-r hover:dark:bg-neutral-800 from-transparent dark:to-black to-neutral-100 hover:bg-neutral-300 bg-opacity-20 cursor-pointer py-2 px-4 rounded-md my-1 drop-shadow-sm" onClick={() => router.push('/driver/' + { constructorId })}>
            <div className="flex gap-8 items-center justify-start">
                <div className="max-w-[8px] w-full">
                    <p className="dark:text-white text-black text-lg font-semibold">{position}</p>
                </div>
                <div className="w-10 h-7 drop-shadow-md flex justify-center">
                    <Flag code={countryCode} className="rounded-sm object-cover" />
                </div>
            </div>
            <div className="flex items-center min-w-[160px] justify-start">
                <p className="dark:text-white text-black">{name}</p>
            </div>
            <div className="flex items-center min-w-[160px] justify-end">
                <p className="dark:text-white text-black">{points}</p>
            </div>

        </div>
    )
}