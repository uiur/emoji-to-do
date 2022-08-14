import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App'
import 'vite/modulepreload-polyfill'
import './index.css'

import {
  BrowserRouter,
  Routes,
  Route,
} from "react-router-dom";


const root = ReactDOM.createRoot(
  document.getElementById("root")!
)

function Login() {
  return (
    <div className="container mx-auto">
      <h1 className="text-3xl font-bold underline">emoji-to-do</h1>
      <div>fuck!</div>
    </div>
  )
}
root.render(
  <BrowserRouter>
    <Routes>
      <Route path="/" element={<App />} />
      <Route path="/login" element={<Login />}/>
    </Routes>
  </BrowserRouter>
);
