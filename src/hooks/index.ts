import { useQuery, useQueryClient } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api";
import { useSelectedAdapter } from "../AdapterContext";
import { UnlistenFn, listen } from "@tauri-apps/api/event"
import { useEffect, useState } from "react";
import { DeviceStatus, DeviceStatusEvent } from "../types";

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



export function useBedJetSubscription() {
  const queryClient = useQueryClient();
  const { selectedAdapter } = useSelectedAdapter();
  const [isListening, setIsListening] = useState(false);


  useEffect(() => {
    let unlisten: UnlistenFn | undefined;

    async function fetchAndListen() {

      unlisten = await listen<DeviceStatusEvent>('DeviceStatus', (event) => {
        queryClient.setQueryData<any>([selectedAdapter, "bedjets", event.payload.id, "status"], event.payload.status)
      });
      setIsListening(true);

    }
    fetchAndListen();

    return () => {
      if (unlisten) unlisten();
      setIsListening(false)
    }
  }, [selectedAdapter, queryClient]);

  return { isListening, }
}


export function useBedJetStatus(id: string) {
  const { selectedAdapter } = useSelectedAdapter();
  const { isListening } = useBedJetSubscription()
  return useQuery<DeviceStatus | undefined>(
    {
      queryKey: [selectedAdapter, "bedjets", id, "status"],
      queryFn: async () => undefined,
      staleTime: Infinity,
      enabled: isListening
    }
  )
}