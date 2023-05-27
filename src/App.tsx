import "./App.css";
import { Button, List, Loader, Paper, Select } from "@mantine/core";
import { useBedJets } from "./hooks";
import { BluetoothAdapterSelect } from "./components/BluetoothAdapterSelect";

function App() {
  const devices = useBedJets();
  return (
    <Paper>
      <BluetoothAdapterSelect />

      {
        devices.isLoading
          ? <Loader />
          : <List>
            {devices.data?.map((device, index) => <List.Item key={index}>{device}</List.Item>)}
          </List>



      }


    </Paper>
  )



}

export default App;
