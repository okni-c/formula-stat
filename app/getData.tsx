'use client'

import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

export default function GetData() {
  const [data, setData] = useState('');

  useEffect(() => {
    invoke<string>('get_data')
      .then((response) => {
        setData(response);
      })
      .catch(console.error);
  }, []);

  return <h4 className='text-emerald-500'>{data}</h4>;
}
