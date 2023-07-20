interface RolexTypes {
    nextEvent: {
        fp1_date: string,
        fp1_time: string,
        fp2_date: string,
        fp2_time: string,
        fp3_date: string,
        fp3_time: string,
        quali_time: string,
        quali_date: string,
        sprint_date: string,
        sprint_time: string,
        time: string,
        date: string,
    }
}

export default function HomePageRolex({ nextEvent }: RolexTypes) {
    return (
        <section className="flex justify-between mt-28">
            <div className="flex flex-col gap-4 w-full">
                <div>
                    <img src="" className="w-20 h-20" />
                    <p>No future dates found.</p>
                </div>
                <h3 className="text-white">FP1</h3>
                <div className="flex items-center justify-between">
                    <p className="text-5xl bg-clip-text bg-gradient-to-b text-transparent from-white to-neutral-300 font-black">{nextEvent.fp1_time}</p>
                    <p className="text-4xl bg-clip-text bg-gradient-to-b text-transparent from-white to-neutral-400 font-regular">{nextEvent.fp1_date}</p>
                </div>
                <h3 className="text-white">FP2</h3>
                <div className="flex items-center justify-between">
                    <p className="text-5xl bg-clip-text bg-gradient-to-b text-transparent from-white to-neutral-300 font-black">{nextEvent.fp2_time}</p>
                    <p className="text-4xl bg-clip-text bg-gradient-to-b text-transparent from-white to-neutral-400 font-regular">{nextEvent.fp2_date}</p>
                </div>
                <h3 className="text-white">FP3</h3>
                <div className="flex items-center justify-between">
                    <p className="text-5xl bg-clip-text bg-gradient-to-b text-transparent from-white to-neutral-300 font-black">{nextEvent.fp3_time}</p>
                    <p className="text-4xl bg-clip-text bg-gradient-to-b text-transparent from-white to-neutral-400 font-regular">{nextEvent.fp3_date}</p>
                </div>
                <h3 className="text-white">Sprint</h3>
                <div className="flex items-center justify-between">
                    <p className="text-5xl bg-clip-text bg-gradient-to-b text-transparent from-white to-neutral-300 font-black">{nextEvent.sprint_time}</p>
                    <p className="text-4xl bg-clip-text bg-gradient-to-b text-transparent from-white to-neutral-400 font-regular">{nextEvent.sprint_date}</p>
                </div>
                <h3 className="text-white">Qualifying</h3>
                <div className="flex items-center justify-between">
                    <p className="text-5xl bg-clip-text bg-gradient-to-b text-transparent from-white to-neutral-300 font-black">{nextEvent.quali_time}</p>
                    <p className="text-4xl bg-clip-text bg-gradient-to-b text-transparent from-white to-neutral-400 font-regular">{nextEvent.quali_date}</p>
                </div>
                <h3 className="text-white">Gran Prix</h3>
                <div className="flex items-center justify-between">
                    <p className="text-5xl bg-clip-text bg-gradient-to-b text-transparent from-white to-neutral-300 font-black">{nextEvent.time}</p>
                    <p className="text-4xl bg-clip-text bg-gradient-to-b text-transparent from-white to-neutral-400 font-regular">{nextEvent.date}</p>
                </div>
            </div>
        </section>
    );
}