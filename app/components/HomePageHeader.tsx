interface HeaderTypes {
    round: number,
    circuitName: string,
    trackImg?: string,
    removeImg?: boolean
}

export default function HomePageHeader({round, circuitName, trackImg, removeImg}: HeaderTypes) {
    return (
        <section className="flex justify-between mt-28">
            <div className="flex flex-col gap-4">
                <h1 className="text-3xl bg-clip-text bg-gradient-to-b text-transparent from-white to-neutral-500 font-black drop-shadow-md">2023 Season</h1>
                <p className="text-5xl bg-clip-text bg-gradient-to-b text-transparent from-white to-neutral-300 font-black">Next Round - {round}</p>
                <p className="text-4xl bg-clip-text bg-gradient-to-b text-transparent from-white to-neutral-400 font-regular">{circuitName}</p>
            </div>
            {removeImg ? null : <img src={trackImg ? trackImg : './f1_miami.png'} className="h-36 aspect-video drop-shadow-lg brightness-0 dark:brightness-100" /> }
        </section>
    );
}