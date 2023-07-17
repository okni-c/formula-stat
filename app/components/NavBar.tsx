"use client"

import Link from "next/link";
import { usePathname } from 'next/navigation'
// import { appWindow } from "@tauri-apps/api/window"
// import { useEffect } from "react";

export default function NavBar() {
    const pathname = usePathname();

    // Check if the current route matches a specific path
    const isActiveRoute = (path: string) => {
        return pathname === path;
    };
    // Function to handle changing window title bar to the current route and format the text
    // useEffect(() => {
    //     async function updateTitle() {
    //         try {
    //             // Format Pathname for Titlebar
    //             const pathConvert = (path: string) => {
    //                 // Remove the leading slash and convert to lowercase
    //                 const formattedPath = path.slice(1).toLowerCase();
    //                 // Capitalize the first letter
    //                 const formattedText = formattedPath.charAt(0).toUpperCase() + formattedPath.slice(1);
    //                 return formattedText;
    //             }
    //             await appWindow.setTitle(pathConvert(pathname))
    //         } catch (error) {
    //             console.error(error)
    //         }
    //     }
    //     updateTitle()
    // }, [pathname])

    return (
        <nav className="z-[2] w-full max-w-[9rem] h-screen shadow-lg dark:bg-neutral-900 bg-white fixed">
            <div className="flex flex-col justify-start h-[100%]">
                <Link href="/" className="mb-[61px] mt-5"><img src="/f1-logo.png" alt="F1 Logo" width={107} height={62} className=" ml-3 brightness-0 dark:invert-[1] invert-0" /></Link>

                <Link href="/races" className="hover:dark:bg-zinc-300 hover:bg-zinc-200 hover:text-black dark:text-white text-black text-xl pl-6 py-4 duration-150 ease-in-out"><span className={pathname.startsWith('/races') ? 'font-black' : ''}>Races</span></Link>

                <Link href="/drivers" className="hover:dark:bg-zinc-300 hover:bg-zinc-200 hover:text-black dark:text-white text-black text-xl font-normal pl-6 py-4 duration-150 ease-in-out"><span className={isActiveRoute('/drivers') ? 'font-black' : ''}>Drivers</span></Link>

                <Link href="/tracks" className="hover:dark:bg-zinc-300 hover:bg-zinc-200 hover:text-black dark:text-white text-black text-xl font-normal pl-6 py-4 duration-150 ease-in-out"><span className={isActiveRoute('/circuts') ? 'font-black' : ''}>Circuits</span></Link>

                <Link href="/settings" className="hover:dark:bg-zinc-300 hover:bg-zinc-200 hover:text-black dark:text-white text-black text-xl font-normal pl-6 py-4 duration-150 ease-in-out justify-end mt-auto"><span className={isActiveRoute('/settings') ? 'font-black' : ''}>Settings</span></Link>
            </div>
        </nav>
    );
}