import { AxiosError } from "axios"
import React from "react"
import { useEffect } from "react"
import { useLocation } from "react-router-dom"
import useSWR from "swr"
import { client } from "../api/client"

const fetcher = async (url: string) => {
  const res = await client.get(url)

  return res.data
}

function NotFound() {
  return (
    <div>
      <h1>Not Found</h1>
    </div>
  )
}

function Loading() {
  return (
    <div>
      <h1>Loading...</h1>
    </div>
  )
}

export function Sandbox() {
  // useEffect(() => {
  //   (async () => {
  //     const res = await client.get('https://httpbin.org/status/404')
  //     const { url } = res.data
  //   })()
  // }, [])

  const { search } = useLocation()
  let query = new URLSearchParams(search)
  let status = query.get('status') || '200'

  const { data, error } = useSWR<unknown, AxiosError>(`https://httpbin.org/status/${status}`, fetcher)

  if (error) {
    if (error.response) {
      if (error.response.status === 404) {
        return (
          <NotFound />
        )
      }
    }

    return (
      <div>
        { error.message }
      </div>
    )
  }

  if (data === undefined) {
    return (
      <Loading />
    )
  }

  return (
    <div>
      ok
    </div>
  )
}
