export default function RacePageHeader() {
    return (
        <section className="flex justify-between mt-28">
            <div className="flex flex-col gap-5">
                <h1 className="text-6xl dark:text-white text-black font-bold drop-shadow-md">Up next:</h1>
                <h2 className="text-6xl dark:text-white text-black font-light">Miami Qualifying</h2>
            </div>
            <img src="f1_miami.png" className="h-36 aspect-video drop-shadow-lg brightness-0 dark:brightness-100" />
        </section>
    );
}