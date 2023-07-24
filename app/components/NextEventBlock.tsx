'use client'

import Countdown, { zeroPad } from 'react-countdown';
import RolexClock from './RolexClock/RolexClock';
import formatDateTime from '../hooks/formateDateTime';
import { useState } from 'react';
import NextEventDropDown from './NextEventDropDown';

interface EventTypes {
    nextEvent: {
        next_event_name: string,
        next_event_time: string,
        grand_prix_name: string,
        grand_prix_date: string,
        grand_prix_time: string,
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
    }
}

export default function NextEventBlock({ nextEvent }: EventTypes) {
    const [open, setOpen] = useState<Boolean>(false);
    const Counter = () => <span className="ml-auto text-3xl bg-clip-text bg-gradient-to-b text-transparent from-red-400 to-red-800 font-black">Live</span>;

    // Renderer callback with condition
    const renderer = ({ days, hours, minutes, seconds, completed }: any) => {
        if (completed) {
            return <Counter />;
        } else {
            return <span className="ml-auto text-3xl bg-clip-text bg-gradient-to-b text-transparent from-black dark:from-white dark:to-neutral-300 to-neutral-600 font-black">{days}:{zeroPad(hours)}:{zeroPad(minutes)}:{zeroPad(seconds)}</span>;
        }
    };

    const timeLeft = new Date(nextEvent.next_event_time).getTime() - Date.now();

    const CountDownTimer = () => <Countdown date={Date.now() + timeLeft} renderer={renderer} />;

    return (
        <section className="flex flex-col mt-14">
            {/* Original Box */}
            <div className="bg-opacity-80 bg-white dark:bg-neutral-900 solid-opacity-70 rounded-3xl p-3 w-full cursor-pointer" onClick={() => { open ? setOpen(false) : setOpen(true) }}>
                <div className='flex w-full items-center gap-4'>
                    <RolexClock />
                    <div>
                        <p className="text-3xl bg-clip-text bg-gradient-to-b text-transparent from-black dark:from-white dark:to-neutral-300 to-neutral-600 font-regular">{nextEvent.next_event_name}</p>
                        <p className="text-lg bg-clip-text bg-gradient-to-b text-transparent from-black dark:from-white dark:to-neutral-500 to-neutral-600 font-regular">{formatDateTime(nextEvent.next_event_time)}</p>
                    </div>
                    <CountDownTimer />
                </div>
                {/* Toggled Box on click */}
                {open ? <NextEventDropDown title='Sub Events' nextEvent={nextEvent} /> : null}
            </div>
        </section>
    );
}