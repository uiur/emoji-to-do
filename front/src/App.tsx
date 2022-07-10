import { useEffect, useState } from 'react'
import logo from './logo.svg'
import './App.css'
import axios from 'axios'

const client = axios.create({});
interface User {
  id: string,
  slack_user_id: string,
  slack_team_id: string,
}

function App() {
  const [token, setToken] = useState(null)
  const [user, setUser] = useState<User | null>(null)

  useEffect(() => {
    (async () => {
      const res = await client.get('/api/token')
      setToken(res.data.token)
    })()
  }, [])

  useEffect(() => {
    (async () => {
      if (!token) return
      const res = await client.get('/api/user', {
         headers: {
          Authorization: `Bearer ${token}`
        }
      })
      setUser(res.data)
    })()
  }, [token])

  return (
    <div>
      <h1>emoji-to-do</h1>
      {
        user && (
          <div>
            <p>{user.id}</p>
            <p>{user.slack_user_id}</p>
            <p>{user.slack_team_id}</p>
          </div>
        )
      }
      <a href='/auth/slack'>Login with Slack</a>
    </div>
  )
}

export default App
