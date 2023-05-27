import { createContext, useContext, useEffect, useState, ReactNode } from 'react';
import { useAdapters } from './hooks';
import { LoadingOverlay } from '@mantine/core';
import { useLocalStorage } from '@mantine/hooks';

type AdapterContextValue = {
  selectedAdapter: string | undefined;
  setSelectedAdapter: React.Dispatch<React.SetStateAction<string | undefined>>;
};

const SelectedAdapterContext = createContext<AdapterContextValue | undefined>(undefined);


interface SelectedAdapterProviderProps {
  children: ReactNode;
}

export const SelectedAdapterProvider = ({ children }: SelectedAdapterProviderProps) => {
  const { data: adapters, isLoading, isError } = useAdapters();

  const [selectedAdapter, setSelectedAdapter] = useLocalStorage<string | undefined>({
    key: "selectedAdapter",
    defaultValue: undefined,
  })

  useEffect(() => {
    if (selectedAdapter && !adapters?.includes(selectedAdapter) && adapters?.length) {
      setSelectedAdapter(adapters[0]);
    }
  }, [adapters]);


  if (isLoading || isError) {
    return <LoadingOverlay visible={true} />
  }

  return (
    <SelectedAdapterContext.Provider value={{ selectedAdapter, setSelectedAdapter }}>
      {children}
    </SelectedAdapterContext.Provider>
  );
};

// Custom hook to use the SelectedAdapterContext
export const useSelectedAdapter = (): AdapterContextValue => {
  const context = useContext(SelectedAdapterContext);

  if (!context) {
    throw new Error('useSelectedAdapter must be used within a SelectedAdapterProvider');
  }

  return context;
};
