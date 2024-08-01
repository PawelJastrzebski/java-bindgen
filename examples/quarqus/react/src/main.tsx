import React from 'react'
import ReactDOM from 'react-dom/client'
import Router from './Router.tsx'
import './index.scss'

import { NextUIProvider } from "@nextui-org/react";

ReactDOM.createRoot(document.getElementById('root')!).render(
  <NextUIProvider>
    <React.StrictMode>
      <main className="dark text-foreground bg-background">
        <Router />
      </main>
    </React.StrictMode>
  </NextUIProvider>
)
