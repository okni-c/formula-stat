import { HeaderTypes } from "../interfaces/interfaces";

export default function HomePageHeader({round, circuitName, trackImg, removeImg}: HeaderTypes) {
    return (
        <section className="flex justify-between mt-20">
            <div className="flex flex-col gap-2">
                <h1 className="text-3xl bg-clip-text bg-gradient-to-b text-transparent from-black dark:from-white dark:to-neutral-400 to-neutral-600 font-black  leading-snug">2023 Season</h1>
                <p className="text-5xl bg-clip-text bg-gradient-to-b text-transparent from-black dark:from-white dark:to-neutral-200 to-neutral-700 font-black leading-snug">Next Round - {round}</p>
                <p className="text-4xl bg-clip-text bg-gradient-to-b text-transparent from-black dark:from-white dark:to-neutral-300 to-neutral-600 font-regular leading-snug">{circuitName}</p>
            </div>
            {removeImg ? null : <img src={trackImg ? trackImg : './f1_miami.png'} className="h-36 aspect-video  brightness-0 dark:brightness-100" /> }
        </section>
    );
}