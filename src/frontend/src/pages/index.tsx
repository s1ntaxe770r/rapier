import Head from 'next/head'
import Image from 'next/image'
import { Inter } from '@next/font/google'
import styles from '@/styles/Home.module.css'

const inter = Inter({ subsets: ['latin'] })

export default function Home() {
  return (
    <>
      <Head>
        <title>Rapier</title>
        <meta name="description" content="The region aware API" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/favicon.ico" />
      </Head>

    
    <h1 className='text-center heading text-5xl mt-8 tracking-widest'>
     Rapier 
    </h1>

    <p className='text-center text-xl mt-4 white'>
      The region aware API
    </p>



    <div className='flex flex-col items-center justify-center mt-10'>
     
    </div>    
    </>
  )
}
