'use client'

import Countdown, { zeroPad } from 'react-countdown';
import RolexClock from './RolexClock/RolexClock';
import { formatDateTime } from '../hooks/dateFormatter';
import { useState } from 'react';
import NextEventDropDown from './NextEventDropDown';
import { AnimatePresence, motion } from 'framer-motion';
import { EventTypes } from '../interfaces/interfaces';

export default function NextEventBlock({ nextEvent }: EventTypes) {
    const [open, setOpen] = useState<Boolean>(false);
    const Counter = () => <span className="ml-auto text-3xl bg-clip-text bg-gradient-to-b text-transparent from-red-400 to-red-800 font-black flex items-center"><div className='w-4 h-4 rounded-full bg-gradient-to-b from-red-400 to-red-800 mr-2 animate-pulse'></div>Live</span>;

    // Renderer callback with condition
    const renderer = ({ days, hours, minutes, seconds, completed }: any) => {
        if (completed) {
            return <Counter />;
        } else {
            return <span className="ml-auto text-3xl bg-clip-text bg-gradient-to-b text-transparent from-black dark:from-white dark:to-neutral-300 to-neutral-600 font-black">{days}:{zeroPad(hours)}:{zeroPad(minutes)}:{zeroPad(seconds)}</span>;
        }
    };
    const dateObject = new Date(nextEvent.next_event_time);
    const localTimeOffsetMinutes = dateObject.getTimezoneOffset();
    dateObject.setMinutes(dateObject.getMinutes() - localTimeOffsetMinutes);
    const timeLeft = dateObject.getTime() - Date.now();

    const CountDownTimer = () => <Countdown date={Date.now() + timeLeft} renderer={renderer} />;

    return (
        <section className="flex flex-col mt-14 gap-4">
            <div className="relative flex flex-col items-center justify-center px-6 py-3 overflow-hidden font-bold rounded-2xl shadow-2xl group bg-white cursor-pointer select-none" onClick={() => { open ? setOpen(false) : setOpen(true) }}>
                <span className="absolute z-0 inset-0 w-full h-full transition duration-300 ease-out opacity-0 bg-gradient-to-br dark:from-neutral-600 dark:via-neutral-700 dark:to-neutral-400 from-zinc-100 via-zinc-200 to-zinc-100 group-hover:opacity-40"></span>

                <span className="absolute z-0 top-0 left-0 w-full bg-gradient-to-b from-white to-transparent opacity-5 h-1/3"></span>

                <span className="absolute z-0 bottom-0 left-0 w-full h-1/3 bg-gradient-to-t from-white to-transparent opacity-5"></span>

                <span className="absolute z-0 bottom-0 left-0 w-4 h-full bg-gradient-to-r from-white to-transparent opacity-5"></span>

                <span className="absolute z-0 bottom-0 right-0 w-4 h-full bg-gradient-to-l from-white to-transparent opacity-5"></span>
                <span className="absolute z-0 inset-0 w-full h-full border border-white rounded-md opacity-10"></span>



                <div className='relative z-20 flex w-full items-center gap-4'>
                    <RolexClock />
                    <div>
                        <p className="text-3xl bg-clip-text bg-gradient-to-b text-transparent from-black dark:from-white dark:to-neutral-300 to-neutral-600 font-regular">{nextEvent.next_event_name}</p>
                        <p className="text-lg bg-clip-text bg-gradient-to-b text-transparent from-black dark:from-white dark:to-neutral-500 to-neutral-600 font-regular">{formatDateTime(nextEvent.next_event_time)}</p>
                    </div>
                    <CountDownTimer />
                </div>

                {/* Toggled Box on click */}
                <AnimatePresence>
                    {open &&
                        <motion.div key="nextEventDropdown" className='w-full'
                            initial={{ height: 0 }}
                            animate={{ height: "auto" }}
                            exit={{ height: 0 }}
                            transition={{ duration: 0.3 }}>
                            <NextEventDropDown title='Sub Events' nextEvent={nextEvent} />
                        </motion.div>
                    }
                </AnimatePresence>
            </div>
        </section>
    );
}