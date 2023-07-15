'use client'

import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

interface RecordTypes {
    id: number,
    source: string,
}

export default function SendData() {
    const [data, setData] = useState<RecordTypes | any>([]);

    useEffect(() => {
        invoke<Object>('send_all_data')
            .then((response) => {
                setData(response);
            })
            .catch(console.error);
    }, []);

    return (
        <>
            {data && data.map((data:RecordTypes) =>
                <div key={data.id} className='flex flex-col border-2 border-zinc-900 rounded-lg p-3 m-3'>
                    <h4 className='text-red-500'>ID: {data.id}</h4>
                    <h4 className='text-red-500'>Name: {data.source}</h4>
                </div>
            )}
        </>
    );
}
