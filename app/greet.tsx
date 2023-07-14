'use client'

import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

export default function Greet() {
  const [greeting, setGreeting] = useState('');

  useEffect(() => {
    invoke<string>('greet', { name: 'Next.js' })
      .then((response) => {
        setGreeting(response);
      })
      .catch(console.error);
  }, []);

  return <h4 className='text-amber-500'>{greeting}</h4>;
}
