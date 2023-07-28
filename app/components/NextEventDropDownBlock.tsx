import { useEffect, useState } from "react";
import { NextEventDropDownBlockTypes } from "../interfaces/interfaces";
import { formatDate } from "../hooks/dateFormatter";

export default function NextEventDropDownBlock({ circuitName, date, time }: NextEventDropDownBlockTypes) {
    const [sTime, setSTime] = useState('');
    const [sDay, setSDay] = useState('');
    const [sDate, setSDate] = useState('');

    useEffect(() => {
        setSTime(formatDate(date, time, "time"));
        setSDate(formatDate(date, time));
        setSDay(formatDate(date, time, "day"));
    }, [date, time])

    return (
        <div className="flex gap-6 justify-between items-center bg-gradient-to-r from-transparent dark:to-black to-neutral-100 bg-opacity-20 py-2 px-4 rounded-md my-1 drop-shadow-sm">
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
    );
}