import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./styles.css";
import { MantineProvider } from '@mantine/core';
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { SelectedAdapterProvider } from "./AdapterContext";

const queryClient = new QueryClient()

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <QueryClientProvider client={queryClient}>
      <SelectedAdapterProvider>
        <MantineProvider withCSSVariables withGlobalStyles withNormalizeCSS>
          <App />
        </MantineProvider>
      </SelectedAdapterProvider>
    </QueryClientProvider>
  </React.StrictMode>
);
