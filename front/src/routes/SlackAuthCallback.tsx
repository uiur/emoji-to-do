import { useEffect } from "react"
import { useLocation } from "react-router-dom";
import { client } from "../api/client"

export function SlackAuthCallback() {
  const { search } = useLocation();
  let query = new URLSearchParams(search)
  let code = query.get('code')

  useEffect(() => {
    (async () => {
      await client.get('/auth/slack/callback', {
        params: {
          code
        }
      })
      location.href = '/'
    })()

  }, [])
  return (
    <div>
    </div>
  )
}
