"use client"

import Link from "next/link";
import { usePathname } from 'next/navigation'

export default function NavBar() {
    const pathname = usePathname();

    // Check if the current route matches a specific path
    const isActiveRoute = (path: string) => {
        return pathname === path;
    };

    return (
        <nav className="w-full max-w-[9rem] h-screen shadow-lg dark:bg-neutral-900 bg-white">
            <div className="flex flex-col justify-start h-[100%]">
                <Link href="/" className="mb-[61px] mt-5"><img src="/f1-logo.png" alt="F1 Logo" width={107} height={62} className=" ml-3 brightness-0 dark:invert-[1] invert-0" /></Link>

                <Link href="/races" className="hover:dark:bg-zinc-300 hover:bg-zinc-200 hover:text-black dark:text-white text-black text-xl pl-6 py-4 duration-150 ease-in-out"><span className={isActiveRoute('/races') ? 'font-black' : ''}>Races</span></Link>

                <Link href="/drivers" className="hover:dark:bg-zinc-300 hover:bg-zinc-200 hover:text-black dark:text-white text-black text-xl font-normal pl-6 py-4 duration-150 ease-in-out"><span className={isActiveRoute('/drivers') ? 'font-black' : ''}>Drivers</span></Link>

                <Link href="/tracks" className="hover:dark:bg-zinc-300 hover:bg-zinc-200 hover:text-black dark:text-white text-black text-xl font-normal pl-6 py-4 duration-150 ease-in-out"><span className={isActiveRoute('/tracks') ? 'font-black' : ''}>Tracks</span></Link>

                <Link href="/settings" className="hover:dark:bg-zinc-300 hover:bg-zinc-200 hover:text-black dark:text-white text-black text-xl font-normal pl-6 py-4 duration-150 ease-in-out justify-end mt-auto"><span className={isActiveRoute('/settings') ? 'font-black' : ''}>Settings</span></Link>
            </div>
        </nav>
    );
}