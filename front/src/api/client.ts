import axios, { AxiosError, AxiosResponse } from "axios";

export const client = axios.create({
  baseURL: 'https://dev-api.emoji-to-do.com',
  withCredentials: true
})

interface Result {
  res: AxiosResponse<any, any> | null
  err: AxiosError | null
}

function wrapResponse(promise: Promise<AxiosResponse<any, any>>): Promise<Result> {
  return promise.then(res => ({ res, err: null})).catch(err => ({ res: null, err }))
}

export function apiPost(url: string, data: any = {}) {
  return wrapResponse(client.post(url, data))
}
export function apiDelete(url: string, data: any = {}) {
  return wrapResponse(client.delete(url, data))
}
