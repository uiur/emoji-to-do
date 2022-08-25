import { FormEvent, useCallback, useState } from 'react'
import { ButtonStyle } from '../../components/Button'
import { Reaction } from '../../types/Reaction'

export interface EmojiFormResult {
  name: string
  repo: string
  assignees: string[]
}

export function EmojiForm({ reaction = null, buttonText = 'Save', onSubmit }: { reaction: Reaction | null, buttonText: string, onSubmit: (data: EmojiFormResult) => void}) {
  const [name, setName] = useState(reaction?.name || '')
  const [repo, setRepo] = useState(reaction?.repo || '')
  const [assignees, setAssignees] = useState<string[]>(
    reaction?.reaction_assignees?.map(reaction_assignee => reaction_assignee.name) || ['']
  )
  const formOnSubmit = useCallback((e: FormEvent) => {
      e.preventDefault()
      onSubmit({ name, repo, assignees })
  }, [name, repo, assignees])

  return (
    <form
      className="flex-1 flex flex-col space-y-4"
      onSubmit={formOnSubmit}
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

      <div className="flex flex-col">
        <label className="text-md font-bold mb-2">assignees</label>
        {assignees.map((assignee, index) => {
          return (
            <div key={index} className="flex flex-row mb-2">
              <input
                key={index}
                className="flex-1 border-2 border-stone-200 rounded px-3 py-2"
                type="text"
                value={assignee}
                onChange={(e) => {
                  const newAssignees = assignees.slice()
                  newAssignees[index] = e.currentTarget.value
                  setAssignees(newAssignees)
                }}
              />

              <button
                className="w-12"
                onClick={() => {
                  const newAssignees = assignees.slice()
                  newAssignees.splice(index, 1)
                  setAssignees(newAssignees)
                }}
              >
                -
              </button>
            </div>
          )
        })}
        <button
          className={[ButtonStyle(), 'mt-2'].join(' ')}
          onClick={(e) => {
            e.preventDefault()
            const newAssignees = assignees.slice()
            newAssignees.push('')
            setAssignees(newAssignees)
          }}
        >
          Add
        </button>
      </div>

      <input type="submit" value={buttonText} className={ButtonStyle()} />
    </form>
  )
}
