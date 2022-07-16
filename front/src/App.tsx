import useSWR from 'swr'
import { createContext, useContext, useEffect, useState } from 'react'
import logo from './logo.svg'
import './App.css'
import axios from 'axios'

const client = axios.create({})
const TokenContext = createContext(null)
interface User {
  id: number
  slack_user_id: string
  slack_team_id: string
}

interface Team {
  id: number
  name: string
  slack_team_id: string
}

interface Reaction {
  id: number
  name: string
  repo: string
}

const fetchWithToken = async (url: string, token: string | null) => {
  if (!token) {
    throw new Error('No token')
  }

  const res = await client.get(url, {
    headers: {
      Authorization: `Bearer ${token}`,
    },
  })

  return res.data
}

function Content() {
  const token = useContext(TokenContext)
  const { data: user } = useSWR<User>('/api/user', (url) =>
    fetchWithToken(url, token)
  )
  const { data: team } = useSWR<Team>('/api/team', (url) =>
    fetchWithToken(url, token)
  )
  const { data: reactions } = useSWR<Reaction[]>(
    team ? `/api/teams/${team.id}/reactions` : null,
    (url) => fetchWithToken(url, token)
  )

  return (
    <div>
      {user && (
        <div>
          <h2>user</h2>
          <p>{user.id}</p>
          <p>{user.slack_user_id}</p>
          <p>{user.slack_team_id}</p>
        </div>
      )}

      {team && (
        <div>
          <h2>team</h2>
          <p>{team.id}</p>
          <p>{team.name}</p>
        </div>
      )}

      <section>
        <h2>reaction</h2>

        <ul>
          {(reactions || []).map((reaction) => {
            return (
              <li key={reaction.id}>
                id: {reaction.id}, name: {reaction.name}
              </li>
            )
          })}
        </ul>
      </section>
    </div>
  )
}

function App() {
  const [token, setToken] = useState(null)

  useEffect(() => {
    ;(async () => {
      const res = await client.get('/api/token')
      setToken(res.data.token)
    })()
  }, [])

  return (
    <div>
      <h1>emoji-to-do</h1>

      {!token && <a href="/auth/slack">Login with Slack</a>}

      <TokenContext.Provider value={token}>
        {token && <Content />}
      </TokenContext.Provider>
    </div>
  )
}

export default App
