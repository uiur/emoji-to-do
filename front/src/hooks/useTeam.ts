import useSWR from "swr";
import { client } from "../api/client";
import { Team } from "../types/Team";
import { User } from "../types/User";

export function useTeam() {
  return useSWR<Team>('/api/team', (url) => client.get(url).then(res => res.data));
}
