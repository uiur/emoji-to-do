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
import { SlackAuth } from './routes/SlackAuth'
import { SlackAuthCallback } from './routes/SlackAuthCallback'
import { GithubAuth } from './routes/GithubAuth'
import { GithubAuthCallback } from './routes/GithubAuthCallback'
import { Swr } from './routes/sandbox/Swr'
import { Form } from './routes/sandbox/Form'
import { Settings } from './routes/Settings'
import { EmojisNew } from './routes/emojis/EmojisNew'
import { EmojisEdit } from './routes/emojis/EmojisEdit'


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
      <Route path="/settings" element={<Settings />}/>
      <Route path="/emojis/new" element={<EmojisNew />}/>
      <Route path="/emojis/:id/edit" element={<EmojisEdit />}/>

      <Route path="/dev/sandbox/swr" element={<Swr />}/>
      <Route path="/dev/sandbox/form" element={<Form />}/>
    </Routes>
  </BrowserRouter>
);
