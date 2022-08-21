import { createContext, useCallback, useContext, useState } from 'react'
import { useLocation } from 'react-router-dom'
import useSWR from 'swr'
import { client, apiPost } from '../../api/client'

type ToastSetter = (message: string) => void
const ToastSetterContext = createContext<ToastSetter>((value) => {})

function SubmitButton({ status }: { status: number }) {
  const setToast = useContext(ToastSetterContext)
  const [loading, setLoading] = useState<boolean>(false)

  const onSubmit = useCallback(
    async (e: React.FormEvent<HTMLFormElement>) => {
      e.preventDefault()

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
    <form onSubmit={onSubmit}>
      <input
        className="cursor-pointer"
        type="submit"
        disabled={loading}
        value={loading ? 'Loading...': `Submit with ${status}`}
      />
    </form>
  )
}
function Content() {
  return (
    <div>
      {[200, 400, 500].map((status) => {
        return <SubmitButton key={status} status={status} />
      })}
    </div>
  )
}

export function Form() {
  const [message, setMessage] = useState<string | null>(null)
  const [timer, setTimer] = useState<number | null>(null)
  const setToast = useCallback(
    (value: string) => {
      if (timer !== null) {
        clearTimeout(timer)
        setTimer(null)
      }
      setMessage(value)
      setTimer(
        setTimeout(() => {
          setMessage(null)
        }, 5000)
      )
    },
    [message]
  )

  return (
    <div>
      {message !== null && (
        <div className="absolute w-full bg-red-500 text-white p-4">
          {message}
        </div>
      )}

      <header className="h-12"></header>

      <ToastSetterContext.Provider value={setToast}>
        <Content></Content>
      </ToastSetterContext.Provider>
    </div>
  )
}
