import useSWR from "swr";
import { client } from "../api/client";
import { User } from "../types/User";

export function useUser() {
  return useSWR<User>('/api/user', (url) => client.get(url).then(res => res.data));
}
