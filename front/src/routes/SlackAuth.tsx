import { useEffect } from "react"
import { client } from "../api/client"

export function SlackAuth() {
  useEffect(() => {
    (async () => {
      const res = await client.get('/auth/slack')
      const { url } = res.data
      location.href = url
    })()

  }, [])
  return (
    <div></div>
  )
}
