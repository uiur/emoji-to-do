import { FormEvent, useCallback, useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { apiPost, client } from '../../api/client'
import { Button, ButtonStyle } from '../../components/Button'
import { Layout } from '../../components/Layout'
import { useToastSetter } from '../../components/ToastProvider'
import { useTeam } from '../../hooks/useTeam'
import { useUser } from '../../hooks/useUser'
import { EmojiForm, EmojiFormResult } from './EmojiForm'


export function EmojisNew() {
  const { data: team } = useTeam()
  const setToast = useToastSetter()
  const navigate = useNavigate()

  const onSubmit = async (data: EmojiFormResult) => {
    if (team === undefined) return
    const { name, repo, assignees } = data
    const { res, err } = await apiPost(`/api/teams/${team.id}/reactions`, {
      name,
      repo,
      reaction_assignees: assignees.map((name) => ({
        name,
      })),
    })

    if (err) {
      setToast(err.message)
    } else {
      navigate('/')
    }
  }

  return (
    <Layout>
      <div className="mb-20">
        <div className="flex flex-row items-center">
          <h2 className="flex-1 text-lg font-bold">Add Emoji</h2>
        </div>

        <div className="flex flex-row mt-4">
          <EmojiForm onSubmit={onSubmit} reaction={null}></EmojiForm>

          <div className="flex-1"></div>
        </div>
      </div>
    </Layout>
  )
}
