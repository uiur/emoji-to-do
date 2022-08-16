import { useEffect } from "react"
import { useLocation } from "react-router-dom";
import client from "../api/client"

function SlackAuth() {
  const { search } = useLocation();
  let query = new URLSearchParams(search)
  let code = query.get('code')

  useEffect(() => {
    (async () => {
      const res = await client.get('/auth/github/callback', {
        params: {
          code
        }
      })
      location.href = '/'
    })()

  }, [])
  return (
    <div></div>
  )
}
export default SlackAuth
