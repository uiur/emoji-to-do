import { useEffect } from "react"
import client from "../api/client"

function SlackAuth() {
  useEffect(() => {
    (async () => {
      const res = await client.get('/auth/slack')
      const { url } = res.data
      location.href = url
    })()

  }, [])
  return (
    <div className="container mx-auto">
      <h1 className="text-3xl font-bold underline">emoji-to-do</h1>
      <p>auth</p>
    </div>
  )
}
export default SlackAuth
