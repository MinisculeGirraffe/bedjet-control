import "./App.css";
import { AppShell, Box, Button, Container, Group, List, Loader, Paper, RingProgress, Select, Slider, Stack } from "@mantine/core";
import { useBedJetStatus, useBedJetSubscription, useBedJets } from "./hooks";
import { BluetoothAdapterSelect } from "./components/BluetoothAdapterSelect";
import { invoke } from "@tauri-apps/api";
import { ButtonCode, DeviceStatus, OperatingMode } from "./types";
import { Command } from "./types";

const upstairs = "99d86151-1751-6ec0-9393-951cae899789"



function App() {
  const devices = useBedJets();
  const a = useBedJetSubscription();

  const status = useBedJetStatus(upstairs);

  const data = status.data as DeviceStatus | undefined

  return (
    <AppShell>
      <Container>
        <Paper>
          <BluetoothAdapterSelect />
          {
            devices.isLoading
              ? <Loader />
              : <List>
                {devices.data?.map((device, index) => {
                  if (device === upstairs) {
                    return (<Box>
                      <Group>
                        <List.Item key={index}>{device}</List.Item>
                        <Button onClick={() => invoke("connect_bedjet", { bedjetid: device })}  >Connect</Button>
                        <Button onClick={() => invoke("disconnect_bedjet", { bedjetid: device })}>Disconnect</Button>

                      </Group>
                      <Container>
                        {data &&
                          <Stack spacing={"xl"}>


                            <Slider disabled={!data} min={66} max={92}
                              marks={[...SliderMarks,
                              { value: (CtoF(data.actual_temp)), label: "Actual" },
                              ]}
                              defaultValue={Math.round(CtoF(data.target_temp))}
                              onChangeEnd={(value) => {
                                setTemperature(device, data, value)
                              }} />


                            <Slider min={5} max={100} step={5} />
                          </Stack>

                        }
                      </Container>

                    </Box>)
                  }

                })}
              </List>
          }


        </Paper>
      </Container>
    </AppShell>
  )

}

async function send_command(bedjetid: string, command: Command) {
  await invoke("send_command", { bedjetid, command })
}

type TempRange = { min: number, max: number };

export const TempRanges: { mode: OperatingMode, range: TempRange }[] = [
  { mode: OperatingMode.Cool, range: { min: 66, max: 79 } },
  { mode: OperatingMode.Dry, range: { min: 80, max: 89 } },
  { mode: OperatingMode.ExtendedHeat, range: { min: 90, max: 92 } },
];


const SliderMarks = Object.entries(TempRanges).map(([key, val]) => ({ value: val.range.min, label: val.mode }))
export const ModeButtonMapping: { [key in OperatingMode]?: ButtonCode } = {
  [OperatingMode.Cool]: ButtonCode.Cool,
  [OperatingMode.Dry]: ButtonCode.Dry,
  [OperatingMode.ExtendedHeat]: ButtonCode.ExternalHeat,
};


export async function setTemperature(id: string, deviceStatus: DeviceStatus, targetTemp: number): Promise<void> {
  let requiredMode: OperatingMode | undefined;
  let minDifference = Number.MAX_VALUE;


  for (const { mode, range } of TempRanges) {
    if (targetTemp >= range.min && targetTemp <= range.max) {
      requiredMode = mode;
      break;
    }
  }


  if (requiredMode === undefined) {
    throw new Error('Invalid target temperature');
  }

  const requiredButton: ButtonCode | undefined = ModeButtonMapping[requiredMode];

  if (requiredButton === undefined) {
    throw new Error('Invalid operating mode');
  }


  if (deviceStatus.operating_mode !== requiredMode) {
    // save the current timer and fan step
    const originalTimer = deviceStatus.remaining_duration;
    const originalFanStep = deviceStatus.fan_step;

    // switch the mode
    await send_command(id, { type: "Button", content: requiredButton });
    // restore the timer and fan step
    await send_command(id, { type: "SetTime", content: { hours: Math.floor(originalTimer.secs / 3600), minutes: Math.floor((originalTimer.secs % 3600) / 60) } });
    await send_command(id, { type: "SetFan", content: { type: "Percent", value: originalFanStep } });
  }

  // set the temperature

  await send_command(id, { type: "SetTemp", content: { "type": "Fahrenheit", value: targetTemp } });

}


function convertFanPercentToStep(percentage: number): number {
  return (percentage / 5) - 1;
}

const CtoF = (temp: number) => temp * 9 / 5 + 32;

function FtoC(f: number): number {
  return (f - 32) * 5 / 9;
}


export default App;
