import { createContext, useCallback, useContext, useState } from 'react'
import { useLocation } from 'react-router-dom'
import useSWR from 'swr'
import { client, apiPost } from '../../api/client'
import { AppHeader } from '../../components/AppHeader'
import { NavigationHeader } from '../../components/NavigationHeader'
import { ToastProvider, useToastSetter } from '../../components/ToastProvider'



function Button({ onSubmit, disabled = false, value  }: { onSubmit: any, value: string, disabled?: boolean }) {
  const handler = (e: any) => {
    e.preventDefault()
    onSubmit(e)
  }

  return (
    <form onSubmit={handler}>
      <input
        className="rounded bg-stone-600 hover:bg-stone-700 disabled:bg-stone-500 text-white px-4 py-3 cursor-pointer"
        type="submit"
        disabled={disabled}
        value={value}
      />
    </form>
  )
}

function SubmitButton({ status }: { status: number }) {
  const setToast = useToastSetter()
  const [loading, setLoading] = useState<boolean>(false)

  const onSubmit = useCallback(
    async (e: React.FormEvent<HTMLFormElement>) => {
      if (loading) return
      setLoading(true)

      const { res, err } = await apiPost(`https://httpbin.org/status/${status}`)

      setLoading(false)

      if (err) {
        setToast(err.message)
      }
    },
    [status]
  )

  return (
    <Button onSubmit={onSubmit} value={`Submit with ${status}`} disabled={loading} />
  )
}

function Content() {
  return (
    <div>
      <div className='mb-20'>
        <div className='flex flex-row items-center'>
          <h2 className='flex-1 text-lg font-bold'>
            Emojis
          </h2>

          <div className='flex-1 flex justify-end'>
            <Button onSubmit={()=>{}} value="Add Emoji"></Button>

          </div>
        </div>
        <div className="flex flex-col">
          <div className='flex flex-row'>
            <div className='flex-1 py-2'>emoji</div>
            <div className='flex-1 py-2'>repo</div>
            <div className='flex-1 py-2'>assign</div>
            <div className='flex-1 py-2'>project</div>
            <div className='flex-1 py-2'></div>
          </div>

          <div className='h-px bg-gray-200'></div>

          {
            [
              ['angel', 'uiur/sandbox', 'uiur', 'Develop'],
              ['angel', 'uiur/sandbox', 'uiur', 'Develop'],
            ].map(([emoji, repo, assign, project], index) => {
              return (
                <div key={index} className='flex flex-row'>
                  <div className='flex-1 py-2'>angel</div>
                  <div className='flex-1 py-2'>uiur/sandbox</div>
                  <div className='flex-1 py-2'>uiur</div>
                  <div className='flex-1 py-2'>Develop</div>
                  <div className='flex-1 py-2 flex justify-end'>
                    <div className='mr-3'>edit</div>
                    <div className='mr-3'>delete</div>
                  </div>
                </div>
              )
            })
          }
        </div>
      </div>
      {[200, 400, 500].map((status) => {
        return (
          <div className='mb-2' key={status}>
            <SubmitButton key={status} status={status} />
          </div>
        )
      })}
    </div>
  )
}


export function Form() {
  return (
    <ToastProvider>
      <AppHeader />
      <NavigationHeader />
      <div className='mx-auto container my-4'>
        <Content></Content>
        <Button onSubmit={()=>{}} value="Login with Slack"></Button>
      </div>
    </ToastProvider>
  )
}
