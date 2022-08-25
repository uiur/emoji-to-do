import { useCallback } from 'react'
import { useNavigate, useParams } from 'react-router-dom'
import { apiPut } from '../../api/client'
import { Layout } from '../../components/Layout'
import { useToastSetter } from '../../components/ToastProvider'
import { useReaction } from '../../hooks/useReaction'
import { EmojiForm, EmojiFormResult } from './EmojiForm'

export function EmojisEdit() {
  const { id } = useParams()
  const { data: reaction, mutate } = useReaction(Number(id))
  const setToast = useToastSetter()

  const navigate = useNavigate()
  const onSubmit = useCallback(async ({ name, repo, assignees }: EmojiFormResult) => {
    if (reaction === undefined) return

    const { res, err } = await apiPut(`/api/reactions/${reaction.id}`, {
      name,
      repo,
      reaction_assignees: assignees.map(name => ({ name }))
    })

    if (err) {
      setToast(err.message)
    } else {
      mutate()
      navigate('/')
    }
  }, [reaction])

  return (
    <Layout>
      <div className="mb-20">
        <div className="flex flex-row items-center">
          <h2 className="flex-1 text-lg font-bold">Edit Emoji</h2>
        </div>

        <div className="flex flex-row mt-4">
          <div className="flex-1">
            {reaction !== undefined && <EmojiForm reaction={reaction} onSubmit={onSubmit}></EmojiForm>}
          </div>

          <div className="flex-1"></div>
        </div>
      </div>
    </Layout>
  )
}
