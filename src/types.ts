export type Duration = {
	nanos: number,
	secs: number
}

export enum OperatingMode {
	Standby = "Standby",
	NormalHeat = "NormalHeat",
	TurboHeat = "TurboHeat",
	ExtendedHeat = "ExtendedHeat",
	Cool = "Cool",
	Dry = "Dry",
	Wait = "Wait",
}

export enum ShutDownCode {
	Normal = "Normal",
	InvalidADC = "InvalidADC",
	ThermistorTrackingError = "ThermistorTrackingError",
	FastOverTempTrip = "FastOverTempTrip",
	SlowOverTempTrip = "SlowOverTempTrip",
	FanFailure = "FanFailure",
	HeaterPowerStandby = "HeaterPowerStandby",
	ExtenderThermalTrip = "ExtenderThermalTrip",
}

export enum UpdateStatus {
	Idle = "Idle",
	Starting = "Starting",
	ConnectingToAP = "ConnectingToAP",
	GotIPAddress = "GotIPAddress",
	CheckingConnection = "CheckingConnection",
	CheckingForUpdate = "CheckingForUpdate",
	Updating = "Updating",
	RestartingBedJet = "RestartingBedJet",
	NoWiFiConfig = "NoWiFiConfig",
	UnableToConnect = "UnableToConnect",
	DHCPFailure = "DHCPFailure",
	UnableToContactServer = "UnableToContactServer",
	ConnectionTestOK = "ConnectionTestOK",
	ConnectionTestFailed = "ConnectionTestFailed",
	NoUpdateNeeded = "NoUpdateNeeded",
	RadioDisabled = "RadioDisabled",
	RestartingBedJetTerminal = "RestartingBedJetTerminal",
	UpdateFailed = "UpdateFailed",
}

export interface DeviceStatus {
	/** The total runtime left on the device */
	remaining_duration: Duration;
	/** Temp in degrees C */
	actual_temp: number;
	/** Temp in degrees C */
	target_temp: number;
	operating_mode: OperatingMode;
	fan_step: number;
	/** Maximum runtime for the current mode */
	max_duration: Duration;
	min_target_temp: number;
	max_target_temp: number;
	ambient_temp: number;
	shutdown_code: ShutDownCode;
	current_update_state: UpdateStatus;
}

export interface DeviceStatusEvent {
	id: string;
	status: DeviceStatus;
}

export type Command =
	| { type: "Button", content: ButtonCode }
	| {
		type: "SetTime", content: {
			hours: number;
			minutes: number;
		}
	}
	| { type: "SetTemp", content: Temp }
	| { type: "SetFan", content: Fan }
	| {
		type: "SetClock", content: {
			hours: number;
			minutes: number;
		}
	};

export type Temp =
	| { type: "Celsius", value: number }
	| { type: "Fahrenheit", value: number };

export type Fan =
	| { type: "Step", value: number }
	| { type: "Percent", value: number };

export enum ButtonCode {
	Stop = "Stop",
	Cool = "Cool",
	Heat = "Heat",
	Turbo = "Turbo",
	Dry = "Dry",
	ExternalHeat = "ExternalHeat",
	FanUp = "FanUp",
	FanDown = "FanDown",
	TempUp1C = "TempUp1C",
	TempDown1C = "TempDown1C",
	TempUp1F = "TempUp1F",
	TempDown1F = "TempDown1F",
	Memory1Recall = "Memory1Recall",
	Memory2Recall = "Memory2Recall",
	Memory3Recall = "Memory3Recall",
	Memory1Store = "Memory1Store",
	Memory2Store = "Memory2Store",
	Memory3Store = "Memory3Store",
	StartConnectionTest = "StartConnectionTest",
	StartFirmwareUpdate = "StartFirmwareUpdate",
	SetLowPowerMode = "SetLowPowerMode",
	SetNormalPowerMode = "SetNormalPowerMode",
	EnableRingOfLight = "EnableRingOfLight",
	DisableRingOfLight = "DisableRingOfLight",
	MuteBeeper = "MuteBeeper",
	UnmuteBeeper = "UnmuteBeeper",
	ResetToFactorySettings = "ResetToFactorySettings",
	EnableWiFiBT = "EnableWiFiBT",
	DisableWiFiBT = "DisableWiFiBT",
	SetConfigCompleteFlag = "SetConfigCompleteFlag",
}

export enum ParameterCode {
	DeviceName = "DeviceName",
	MemoryName1 = "MemoryName1",
	MemoryName2 = "MemoryName2",
	MemoryName3 = "MemoryName3",
	BiorhythmName1 = "BiorhythmName1",
	BiorhythmName2 = "BiorhythmName2",
	BiorhythmName3 = "BiorhythmName3",
	Biorhythm1Fragment1 = "Biorhythm1Fragment1",
	Biorhythm1Fragment2 = "Biorhythm1Fragment2",
	Biorhythm1Fragment3 = "Biorhythm1Fragment3",
	Biorhythm1Fragment4 = "Biorhythm1Fragment4",
	Biorhythm2Fragment1 = "Biorhythm2Fragment1",
	Biorhythm2Fragment2 = "Biorhythm2Fragment2",
	Biorhythm2Fragment3 = "Biorhythm2Fragment3",
	Biorhythm2Fragment4 = "Biorhythm2Fragment4",
	Biorhythm3Fragment1 = "Biorhythm3Fragment1",
	Biorhythm3Fragment2 = "Biorhythm3Fragment2",
	Biorhythm3Fragment3 = "Biorhythm3Fragment3",
	Biorhythm3Fragment4 = "Biorhythm3Fragment4",
	FirmwareVersionCodes = "FirmwareVersionCodes",
}

export enum CommandClass {
	Button = "Button",
	SetTime = "SetTime",
	SetTemp = "SetTemp",
	SetFan = "SetFan",
	SetClock = "SetClock",
	SetParameter = "SetParameter",
}

