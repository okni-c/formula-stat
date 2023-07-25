import { useEffect, useState } from "react";

interface NextEventDropDownBlockTypes {
    circuitName: string,
    date: string,
    time: string,
}

export default function NextEventDropDownBlock({ circuitName, date, time }: NextEventDropDownBlockTypes) {
    const [sTime, setSTime] = useState('');
    const [sDay, setSDay] = useState('');
    const [sDate, setSDate] = useState('');

    useEffect(() => {
        const dateObject = new Date(date + 'T' + time);

        // Format options for date and time
        const dateFormatOptions: any = { month: 'long', day: 'numeric', year: 'numeric' };
        const timeFormatOptions: any = { hour: 'numeric', minute: 'numeric' };
        const dayFormatOptions: any = { weekday: 'long' };

        // Format date and time separately using the local time
        const formattedDate = dateObject.toLocaleDateString(undefined, dateFormatOptions);
        const formattedTime = dateObject.toLocaleTimeString(undefined, timeFormatOptions);
        const formattedDay = dateObject.toLocaleDateString(undefined, dayFormatOptions);

        setSTime(formattedTime);
        setSDate(formattedDate);
        setSDay(formattedDay);
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