import { Select } from "@mantine/core";
import { useSelectedAdapter } from "../AdapterContext";
import { useAdapters } from "../hooks";

export function BluetoothAdapterSelect() {
    const { selectedAdapter, setSelectedAdapter } = useSelectedAdapter()
    const adapters = useAdapters();
    return (
        <Select
        disabled={adapters.isLoading}
        data={adapters.data ?? []}
        onChange={(i) => setSelectedAdapter(i ?? undefined)}
        value={selectedAdapter}
        label="Bluetooth Adapter"
      />
    )
}