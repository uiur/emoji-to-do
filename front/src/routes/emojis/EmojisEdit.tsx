import { useCallback, useEffect, useState } from 'react'
import { Navigate, useNavigate, useParams } from 'react-router-dom'
import { apiPost, apiPut, client } from '../../api/client'
import { Button, ButtonStyle } from '../../components/Button'
import { Layout } from '../../components/Layout'
import { useToastSetter } from '../../components/ToastProvider'
import { useReaction } from '../../hooks/useReaction'
import { useTeam } from '../../hooks/useTeam'
import { useUser } from '../../hooks/useUser'
import { Reaction } from '../../types/Reaction'

function Form({ reaction, submitText, onSave }: { reaction: Reaction, submitText: string, onSave: () => void }) {
  const [name, setName] = useState(reaction.name)
  const [repo, setRepo] = useState(reaction.repo)
  const setToast = useToastSetter()

  const formOnSubmit = useCallback(async () => {
    const { res, err } = await apiPut(`/api/reactions/${reaction.id}`, {
      name,
      repo,
    })

    if (err) {
      setToast(err.message)
    } else {
      onSave()
    }
  }, [name, repo])

  return (
    <form
      className="flex-1 flex flex-col space-y-4"
      onSubmit={(e) => {
        e.preventDefault()
        formOnSubmit()
      }}
    >
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

      <input type="submit" value={submitText} className={ButtonStyle()} />
    </form>
  )
}

export function EmojisEdit() {
  const { id } = useParams()
  const { data: reaction, mutate } = useReaction(Number(id))

  const navigate = useNavigate()
  const onSave = useCallback(() => {
    mutate()
    navigate('/')
  }, [])

  return (
    <Layout>
      <div className="mb-20">
        <div className="flex flex-row items-center">
          <h2 className="flex-1 text-lg font-bold">Edit Emoji</h2>
        </div>

        <div className="flex flex-row mt-4">
          <div className="flex-1">
            {reaction !== undefined && <Form reaction={reaction} submitText='Save' onSave={onSave}></Form>}
          </div>

          <div className="flex-1"></div>
        </div>
      </div>
    </Layout>
  )
}
