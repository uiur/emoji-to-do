import useSWR from 'swr'
import React, {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useState,
} from 'react'
import logo from './logo.svg'
import './App.css'
import axios from 'axios'
import { Link } from 'react-router-dom'
import { client } from './api/client'
import { User } from './types/User'
import { Team } from './types/Team'
import { Reaction } from './types/Reaction'
import { ReactionAssignee } from './types/ReactionAssignee'
import { AppHeader } from './components/AppHeader'
import { NavigationHeader } from './components/NavigationHeader'
import { ToastProvider } from './components/ToastProvider'
import { Layout } from './components/Layout'
import { Button } from './components/Button'

const fetch = async (url: string) => {
  const res = await client.get(url)

  return res.data
}

function ReactionForm({
  team,
  onSave,
}: {
  team: Team
  onSave: (reaction: Reaction) => void
}) {
  const [name, setName] = useState('')
  const [repo, setRepo] = useState('')
  const onSubmit = useCallback(async () => {
    const res = await client.post(`/api/teams/${team.id}/reactions`, {
      name,
      repo,
    })
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

function ReactionAssigneeForm({
  reaction,
  onSave,
}: {
  reaction: Reaction
  onSave: () => void
}) {
  const [name, setName] = useState('')
  const [errorMessage, setErrorMessage] = useState('')
  const onSubmit = useCallback(async () => {
    const res = await client
      .post(`/api/reactions/${reaction.id}/reaction_assignees`, {
        name: name,
      })
      .catch((err) => {
        setErrorMessage(err.message)
      })

    setName('')
    setErrorMessage('')
    onSave()
  }, [reaction.id, name])

  return (
    <form
      className="flex flex-col"
      onSubmit={(e) => {
        e.preventDefault()
        onSubmit()
      }}
    >
      <div className="flex">
        <input
          className="bg-gray-200 p-2 flex-1"
          type="text"
          placeholder="name"
          value={name}
          onChange={(e) => setName(e.currentTarget.value)}
        ></input>

        <input
          className="bg-blue-600 text-white cursor-pointer p-2 ml-2"
          type="submit"
          value="Add"
          disabled={!name}
        ></input>
      </div>

      {errorMessage.length > 0 && (
        <div className="text-red-600">{errorMessage}</div>
      )}
    </form>
  )
}

function ReactionAssigneeComponent({
  reactionAssignee,
  onDelete,
}: {
  reactionAssignee: ReactionAssignee
  onDelete: () => void
}) {
  const deleteOnClick = useCallback(async () => {
    await client.delete(`/api/reaction_assignees/${reactionAssignee.id}`)

    onDelete()
  }, [reactionAssignee.id])

  return (
    <span key={reactionAssignee.id}>
      {reactionAssignee.name}

      <span className="mx-2 cursor-pointer underline" onClick={deleteOnClick}>
        delete
      </span>
    </span>
  )
}

// function Content() {
//   const { data: user } = useSWR<User>('/api/user', fetch)

//   const deleteOnClick = async (reaction: Reaction) => {
//     if (!confirm('Are you sure to delete this reaction?')) return

//     const res = await client.delete(`/api/reactions/${reaction.id}`)
//     mutateReactions()
//   }

//   return (
//     <div>
//       {team?.github_installation_id === null && (
//         <a href="/auth/github">Login with GitHub</a>
//       )}
//       {user && (
//         <div>
//           <h2>user</h2>
//           <p>
//             {user.id} {user.slack_user_id}
//           </p>
//         </div>
//       )}

//       {team && (
//         <div>
//           <h2>team</h2>
//           <p>
//             {team.id} {team.name}
//           </p>
//         </div>
//       )}

//       <section>
//         <h2 className="font-semibold">reaction</h2>

//         <ul className="list-disc list-inside">
//           {(reactions || []).map((reaction) => {
//             return (
//               <li key={reaction.id}>
//                 <div>
//                   <span>
//                     id: {reaction.id}, name: {reaction.name}, repo:{' '}
//                     <a
//                       href={`https://github.com/${reaction.repo}`}
//                       target="_blank"
//                     >
//                       {reaction.repo}
//                     </a>
//                   </span>
//                   <span
//                     className="mx-2 cursor-pointer underline"
//                     onClick={() => deleteOnClick(reaction)}
//                   >
//                     delete
//                   </span>
//                 </div>
//                 <div>
//                   {reaction.reaction_assignees.map((reactionAssignee) => {
//                     return (
//                       <ReactionAssigneeComponent
//                         key={reactionAssignee.id}
//                         reactionAssignee={reactionAssignee}
//                         onDelete={mutateReactions}
//                       />
//                     )
//                   })}
//                   <ReactionAssigneeForm
//                     reaction={reaction}
//                     onSave={mutateReactions}
//                   />
//                 </div>
//               </li>
//             )
//           })}
//         </ul>

//         {team && (
//           <div className="mt-1">
//             <h2 className="font-semibold">create reaction</h2>
//             <ReactionForm
//               team={team}
//               onSave={() => {
//                 mutateReactions()
//               }}
//             />
//           </div>
//         )}
//       </section>
//     </div>
//   )
// }

function Content() {
  const { data: team } = useSWR<Team>('/api/team', fetch)
  const { data: reactions, mutate: mutateReactions } = useSWR<Reaction[]>(
    team ? `/api/teams/${team.id}/reactions` : null,
    fetch
  )

  return (
    <div>
      <div className="mb-20">
        <div className="flex flex-row items-center">
          <h2 className="flex-1 text-lg font-bold">Emojis</h2>

          <div className="flex-1 flex justify-end">
            <Button onSubmit={() => {}} value="Add Emoji"></Button>
          </div>
        </div>
        <div className="flex flex-col">
          <div className="flex flex-row">
            <div className="flex-1 py-2">emoji</div>
            <div className="flex-1 py-2">repo</div>
            <div className="flex-1 py-2">assign</div>
            <div className="flex-1 py-2">project</div>
            <div className="flex-1 py-2"></div>
          </div>

          <div className="h-px bg-gray-200"></div>

          {(reactions || []).map((reaction, index) => {
            return (
              <div key={reaction.id} className="flex flex-row">
                <div className="flex-1 py-2">{reaction.name}</div>
                <div className="flex-1 py-2">{reaction.repo}</div>
                <div className="flex-1 py-2">
                  {reaction.reaction_assignees
                    .map((reactionAssignee) => {
                      reactionAssignee.name
                    })
                    .join(' ')}
                </div>
                <div className="flex-1 py-2"></div>
                <div className="flex-1 py-2 flex justify-end">
                  <div className="mr-3">edit</div>
                  <div className="mr-3">delete</div>
                </div>
              </div>
            )
          })}
        </div>
      </div>
    </div>
  )
}



function App() {
  // const logoutOnClick = useCallback(async () => {
  //   await client.delete('/api/session')
  //   mutate()
  // }, [])

  return (
    <Layout>
      <Content />
    </Layout>
  )
}

export default App
