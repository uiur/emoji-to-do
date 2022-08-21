import { useEffect } from "react"
import { client } from "../api/client"

export function GithubAuth() {
  useEffect(() => {
    (async () => {
      const res = await client.get('/auth/github')
      const { url } = res.data
      location.href = url
    })()

  }, [])
  return (
    <div></div>
  )
}
