'use client'
import Clock from 'react-clock';
import './clock.css';
import { useState, useEffect } from 'react';

export default function RolexClock() {
    const [value, setValue] = useState(new Date());

    useEffect(() => {
        const interval = setInterval(() => setValue(new Date()), 1000);

        return () => {
            clearInterval(interval);
        };
    }, []);

    return (
        <div className='relative'>
            <Clock className="bg-white rounded-full text-[0.55rem]" size={'80px'} value={value} renderNumbers={true} />
        </div>
    )
}
