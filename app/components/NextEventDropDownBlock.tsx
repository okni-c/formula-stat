import { useEffect, useState } from "react";
import { NextEventDropDownBlockTypes } from "../interfaces/interfaces";
import { formatDate } from "../hooks/dateFormatter";

export default function NextEventDropDownBlock({ circuitName, date, time }: NextEventDropDownBlockTypes) {
    const [sTime, setSTime] = useState<string>('');
    const [sDay, setSDay] = useState<string>('');
    const [sDate, setSDate] = useState<string>('');
    const [hidden, setHidden] = useState<boolean>(false);

    useEffect(() => {
        setSTime(formatDate(date, time, "time"));
        setSDate(formatDate(date, time));
        setSDay(formatDate(date, time, "day"));
        if (formatDate(date, time) === 'Invalid Date') {
            setHidden(true)
        }
    }, [date, time])

    return (
        <>
            {!hidden &&
                <div className={'flex gap-6 justify-between items-center bg-gradient-to-r from-transparent dark:to-black to-neutral-100 bg-opacity-20 py-2 px-4 rounded-md my-1 '}>
                    <div className="flex items-center justify-start min-w-[160px]">
                        <p className="dark:text-white text-black">{circuitName}</p>
                    </div>
                    <div className="flex items-center justify-start min-w-[120px]">
                        <p className="dark:text-white text-black">{sDay}</p>
                    </div>
                    <div className="flex items-center justify-start min-w-[120px]">
                        <p className="dark:text-white text-black">{sDate}</p>
                    </div>
                    <div className="flex items-center justify-end min-w-[120px]">
                        <p className="dark:text-white text-black">{sTime}</p>
                    </div>
                </div>
            }
        </>
    );
}