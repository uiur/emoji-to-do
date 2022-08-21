import { useCallback, useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { apiPost, client } from '../../api/client'
import { Button, ButtonStyle } from '../../components/Button'
import { Layout } from '../../components/Layout'
import { useToastSetter } from '../../components/ToastProvider'
import { useTeam } from '../../hooks/useTeam'
import { useUser } from '../../hooks/useUser'

export function EmojisNew() {
  const { data: user } = useUser()
  const { data: team } = useTeam()
  const setToast = useToastSetter()
  const navigate = useNavigate()

  const [name, setName] = useState('')
  const [repo, setRepo] = useState('')
  const onSubmit = useCallback(async () => {
      if (team === undefined) return
      const { res, err } = await apiPost(`/api/teams/${team.id}/reactions`, {
        name,
        repo,
      })
      if (err) {
        setToast(err.message)
      } else {
        setName('')
        setRepo('')
        navigate('/')
      }
    },
    [team, name, repo]
  )

  return (
    <Layout>
      <div className="mb-20">
        <div className="flex flex-row items-center">
          <h2 className="flex-1 text-lg font-bold">Add Emoji</h2>
        </div>

        <div className="flex flex-row mt-4">
          <form className="flex-1 flex flex-col space-y-4" onSubmit={(e) => {e.preventDefault(); onSubmit()}}>
            <div className="flex flex-col">
              <label className="text-md font-bold mb-2">emoji</label>
              <input
                className="border-2 border-stone-200 rounded px-3 py-2"
                type="text"
                value={name}
                onChange={(e) => {
                  setName(e.currentTarget.value)
                }}
              ></input>
            </div>

            <div className="flex flex-col">
              <label className="text-md font-bold mb-2">repo</label>
              <input
                className="border-2 border-stone-200 rounded px-3 py-2"
                type="text"
                value={repo}
                onChange={(e) => {
                  setRepo(e.currentTarget.value)
                }}
              ></input>
            </div>

            <input
              type="submit"
              value="Add Emoji"
              className={ButtonStyle()}
            />
          </form>

          <div className="flex-1"></div>
        </div>
      </div>
    </Layout>
  )
}
