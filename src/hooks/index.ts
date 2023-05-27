import { useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api";
import { useSelectedAdapter } from "../AdapterContext";

export function useAdapters() {
    return useQuery({
        queryKey: ["adapters"],
        queryFn: () => invoke<string[]>("get_btle_adapters"),
        staleTime: Infinity
    })
}

export function useBedJets() {
    const { selectedAdapter } = useSelectedAdapter()
    return useQuery({
      queryKey: [selectedAdapter, "bedjets"],
      queryFn: () => invoke<string[]>("scan_bedjets", { adapter: selectedAdapter }),
      enabled: !!selectedAdapter
    })
  }
  