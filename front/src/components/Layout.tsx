import { useEffect } from "react"
import { Link } from "react-router-dom"
import useSWR from "swr"
import { client } from "../api/client"
import { User } from "../types/User"
import { AppHeader } from "./AppHeader"
import { NavigationHeader } from "./NavigationHeader"
import { ToastProvider, useToastSetter } from "./ToastProvider"

const fetch = async (url: string) => {
  const res = await client.get(url)

  return res.data
}

function LayoutInner({ children }: { children: React.ReactNode }) {
  const { data: user, error } = useSWR<User>('/api/user', fetch)
  const setToast = useToastSetter()
  useEffect(() => {
    if (!error) return
    setToast(error.message)
  }, [error, setToast] )

  return (
    <div className="container mx-auto">
      <AppHeader />
      { user === undefined && <Link to="/auth/slack">Login with Slack</Link>  }
      { user != undefined && (
        <>
          <NavigationHeader />
          { children }
        </>
      ) }
    </div>
  )
}

export function Layout({ children }: { children: React.ReactNode }) {
  return (
    <ToastProvider>
      <LayoutInner>{children}</LayoutInner>
    </ToastProvider>
  )
}
