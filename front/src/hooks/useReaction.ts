import useSWR from "swr";
import { client } from "../api/client";
import { Reaction } from "../types/Reaction";

export function useReaction(id: number | null) {
  return useSWR<Reaction>(id ? `/api/reactions/${id}` : null, (url) => client.get(url).then(res => res.data));
}
