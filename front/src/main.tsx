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
import SlackAuth from './routes/SlackAuth'
import SlackAuthCallback from './routes/SlackAuthCallback'
import GithubAuth from './routes/GithubAuth'
import GithubAuthCallback from './routes/GithubAuthCallback'


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
      <Route path="/auth/slack" element={<SlackAuth />}/>
      <Route path="/auth/slack/callback" element={<SlackAuthCallback />}/>
      <Route path="/auth/github" element={<GithubAuth />}/>
      <Route path="/auth/github/callback" element={<GithubAuthCallback />}/>
      <Route path="/login" element={<Login />}/>
    </Routes>
  </BrowserRouter>
);
