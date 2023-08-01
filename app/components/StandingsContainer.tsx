import { useEffect, useState } from "react";
import { ContainerTypes } from "../interfaces/interfaces"
import { motion } from "framer-motion";

export default function StandingsContainer({ title, children, dropDown }: ContainerTypes) {
    const [animate, isAnimate] = useState<string>('750px');
    // useEffect(() => {
    //     if (dropDown) {
    //         isAnimate('auto')
    //     }
    // }, [dropDown])
    return (
        <motion.section key="ListContainer" className="w-full bg-gradient-to-b dark:from-neutral-900 dark:to-black from-white to-transparent rounded-3xl flex flex-col px-5 py-4 my-10 relative overflow-hidden"
            initial={{ height: '500px' }}
            animate={{ height: animate }}
            exit={{ height: '500px' }}
            transition={{ duration: 0.2 }}>
            <p className="bg-clip-text text-3xl text-transparent font-black bg-gradient-to-b dark:from-white dark:to-neutral-400 from-black to-neutral-700 pb-4 pl-3">{title}</p>
            {children}
            {dropDown &&
                <button className="flex justify-center bottom-0 left-0 absolute z-20 dark:text-neutral-300 dark:hover:text-white text-neutral-800 hover:text-black dark:bg-black bg-white hover:bg-neutral-100 w-full py-4 dark:hover:bg-neutral-900 duration-75 ease-linear rounded-br-3xl rounded-bl-3xl text-sm font-bold" onClick={() => animate === '750px' ? isAnimate('1200px') : isAnimate('750px')}><img src="icons/drop-down-arrow.png" className={animate === '750px' ? 'w-5 rotate-90 dark:invert' : 'w-5 rotate-[270deg] dark:invert'}/></button>
            }
        </motion.section>
    )
}