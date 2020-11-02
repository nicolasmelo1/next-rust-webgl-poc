import React, {useEffect} from 'react'
import Head from 'next/head'
import styles from '../styles/Home.module.css'
const rust = import('../renderer/pkg');

export default function Home() {
  const canvasRef = React.useRef(null)

  useEffect(() => {
    if (document && window) {
      rust
        .then(m => {
          m.get_canvas(canvasRef.current)
        })
        .catch(console.error);
    }
  }, [])
  
  return (
    <div className={styles.container}>
      <Head>
        <title>Create Next App</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main className={styles.main}>
        <canvas ref={canvasRef} width="600" height="600"/>
      </main>
    </div>
  )
}
