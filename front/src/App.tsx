import useSWR, { mutate } from 'swr'
import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useState,
} from 'react'
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

function ReactionForm({
  team,
  onSave,
}: {
  team: Team
  onSave: (reaction: Reaction) => void
}) {
  const token = useContext(TokenContext)
  const [name, setName] = useState('')
  const [repo, setRepo] = useState('')
  const onSubmit = useCallback(async () => {
    const res = await client.post(
      `/api/teams/${team.id}/reactions`,
      { name, repo },
      {
        headers: {
          Authorization: `Bearer ${token}`,
        },
      }
    )
    const reaction = res.data
    onSave(reaction)
    setName('')
    setRepo('')
  }, [name, repo])

  return (
    <form
      className="flex flex-col"
      onSubmit={(e) => {
        e.preventDefault()
        onSubmit()
      }}
    >
      <input
        className="bg-gray-200 p-2"
        type="text"
        placeholder="name"
        value={name}
        onChange={(e) => setName(e.currentTarget.value)}
      ></input>

      <input
        className="bg-gray-200 p-2 mt-2"
        type="text"
        value={repo}
        placeholder="repo"
        onChange={(e) => setRepo(e.currentTarget.value)}
      ></input>

      <input
        className="bg-blue-600 text-white mt-2 cursor-pointer"
        type="submit"
        value="Add"
        disabled={!name || !repo}
      ></input>
    </form>
  )
}

function Content() {
  const token = useContext(TokenContext)
  const { data: user } = useSWR<User>('/api/user', (url) =>
    fetchWithToken(url, token)
  )
  const { data: team } = useSWR<Team>('/api/team', (url) =>
    fetchWithToken(url, token)
  )
  const { data: reactions, mutate: mutateReactions } = useSWR<Reaction[]>(
    team ? `/api/teams/${team.id}/reactions` : null,
    (url) => fetchWithToken(url, token)
  )

  const deleteOnClick = async (reaction: Reaction) => {
    if (!confirm('Are you sure to delete this reaction?')) return

    const res = await client.delete(`/api/reactions/${reaction.id}`, {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    })
    mutateReactions()
  }

  return (
    <div>
      {user && (
        <div>
          <h2>user</h2>
          <p>{user.id} {user.slack_user_id}</p>
        </div>
      )}

      {team && (
        <div>
          <h2>team</h2>
          <p>{team.id} {team.name}</p>
        </div>
      )}

      <section>
        <h2 className="font-semibold">reaction</h2>

        <ul className="list-disc list-inside">
          {(reactions || []).map((reaction) => {
            return (
              <li key={reaction.id}>
                <span>
                  id: {reaction.id}, name: {reaction.name}, repo:{' '}
                  <a
                    href={`https://github.com/${reaction.repo}`}
                    target="_blank"
                  >
                    {reaction.repo}
                  </a>
                </span>
                <span
                  className="mx-2 cursor-pointer underline"
                  onClick={() => deleteOnClick(reaction)}
                >
                  delete
                </span>
              </li>
            )
          })}
        </ul>

        {team && (
          <ReactionForm
            team={team}
            onSave={() => {
              mutateReactions()
            }}
          />
        )}
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
    <div className="container mx-auto">
      <h1 className="text-3xl font-bold underline">emoji-to-do</h1>

      {!token && <a href="/auth/slack">Login with Slack</a>}

      <TokenContext.Provider value={token}>
        {token && <Content />}
      </TokenContext.Provider>
    </div>
  )
}

export default App
