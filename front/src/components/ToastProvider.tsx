import { createContext, useCallback, useContext, useState } from "react"

type ToastSetter = (message: string) => void
const ToastSetterContext = createContext<ToastSetter>((value) => {})
export function useToastSetter() {
  return useContext(ToastSetterContext)
}

export function ToastProvider({ children }: { children: React.ReactNode }) {
  const [message, setMessage] = useState<string | null>(null)
  const [visible, setVisible] = useState<boolean>(false)
  const [timer, setTimer] = useState<number | null>(null)
  const setToast = useCallback(
    (value: string) => {
      if (timer !== null) {
        clearTimeout(timer)
        setTimer(null)
      }
      setVisible(true)
      setMessage(value)
      setTimer(
        setTimeout(() => {
          setVisible(false)
        }, 5000)
      )
    },
    [message]
  )

  return (
    <div>
      <div className="absolute w-full bg-red-500 text-white p-4 transition-opacity duration-500" style={{
        opacity: visible ? 1.0 : 0.0
      }}>
        {message}
      </div>
      <ToastSetterContext.Provider value={setToast}>
        { children }
      </ToastSetterContext.Provider>
    </div>
  )
}
