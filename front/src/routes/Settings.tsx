import { useCallback } from "react"
import { useNavigate } from "react-router-dom"
import { client } from "../api/client"
import { Button } from "../components/Button"
import { Layout } from "../components/Layout"
import { useUser } from "../hooks/useUser"

export function Settings() {
  const { data: user, mutate } = useUser()
  const logoutOnClick = useCallback(async () => {
    await client.delete('/api/session')
    location.href = '/'
  }, [])

  return (
    <Layout>
      <div className='mt-4'>
        <h1 className='text-lg font-bold mb-4'>Settings</h1>
        <Button value='Logout' onSubmit={logoutOnClick} />

      </div>
    </Layout>
  )
}
