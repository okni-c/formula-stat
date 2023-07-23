interface ContainerTypes {
    title: string,
    children: any,
  }

export default function StandingsContainer({ title, children }: ContainerTypes) {

    return (
        <section className="w-full bg-gradient-to-b dark:from-neutral-900 dark:to-black from-white to-transparent rounded-3xl flex flex-col px-5 py-4 my-10">
            <p className="bg-clip-text text-3xl text-transparent font-black bg-gradient-to-b dark:from-white dark:to-neutral-400 from-black to-neutral-700 pb-4 pl-3">{title}</p>
            {children}
        </section>
    )
}