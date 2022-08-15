import { useEffect } from "react"
import { useLocation } from "react-router-dom";
import client from "../api/client"

function SlackAuthCallback() {
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
      // const { url } = res.data
      // location.href = url
    })()

  }, [])
  return (
    <div className="container mx-auto">
      <h1 className="text-3xl font-bold underline">emoji-to-do</h1>
      <p>{ code }</p>
    </div>
  )
}
export default SlackAuthCallback
