import axios from "axios";

const client = axios.create({
  baseURL: 'https://dev-api.emoji-to-do.com',
  withCredentials: true
})

export default client
