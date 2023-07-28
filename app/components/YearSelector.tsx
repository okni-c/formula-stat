"use client"

import { AnimatePresence, motion } from "framer-motion";

import Link from "next/link";
import { usePathname } from 'next/navigation'

export default function YearSelector() {
    const pathname = usePathname();

    const year = [{ year: '2022' }, { year: '2021' }, { year: '2020' }, { year: '2019' }, { year: '2018' }, { year: '2017' }, { year: '2016' }, { year: '2014' }, { year: '2013' }, { year: '2012' }, { year: '2011' }, { year: '2010' }];

    return (
        <>
            <AnimatePresence>
                {pathname.startsWith('/races') &&
                    <motion.nav className="z-[1] w-full max-w-[7rem] h-screen bg-black bg-opacity-20 shadow-lg fixed left-36"
                        key="YearSelector"
                        initial={{ x: -100 }}
                        animate={{ x: 0 }}
                        exit={{ x: -100 }}>
                        <div className="flex flex-col justify-start h-[100%]">

                            <Link href="/races" className="hover:dark:bg-zinc-300 rounded-lg hover:bg-zinc-200 hover:text-black dark:text-white text-black text-lg my-6 py-2 px-5 duration-150 ease-in-out self-center"><span className={pathname === ('/races') ? 'font-black' : ''}>2023</span></Link>

                            <hr className="h-1 opacity-10 bg-zinc-300 mb-2" />

                            {year && year.map((year: any) =>

                                <Link key={year.year} href={'/races/' + year.year} className="hover:dark:bg-zinc-300 rounded-lg hover:bg-zinc-200 hover:text-black dark:text-white text-black text-lg my-1 py-1 px-5 duration-150 ease-in-out self-center"><span className={pathname.startsWith('/races/' + year.year) ? 'font-black' : ''}>{year.year}</span></Link>
                            )}
                        </div>
                    </motion.nav>
                }
            </AnimatePresence>
        </>
    );
}