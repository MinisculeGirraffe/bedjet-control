import React, { useEffect, useState } from 'react';
import './TempSlider.css'; // Importing some custom styles
import { Group, Paper, Text } from '@mantine/core';

interface TemperatureSliderProps {
    min: number,
    max: number,
    actualTemp: number,
    ambientTemp: number,
    targetTemp: number,
    onChange: (value: number) => void,
}

export const TemperatureSlider: React.FC<TemperatureSliderProps> = ({ min, max, actualTemp, ambientTemp, targetTemp, onChange }) => {

    const [localActualTemp, setLocalActualTemp] = useState(actualTemp);
    const [localAmbientTemp, setLocalAmbientTemp] = useState(ambientTemp);
    const [localTargetTemp, setLocalTargetTemp] = useState(targetTemp);

    useEffect(() => {
        setLocalActualTemp(actualTemp);
        setLocalAmbientTemp(ambientTemp);
        setLocalTargetTemp(targetTemp);
    }, [actualTemp, ambientTemp, targetTemp]);

    // Calculate heights of actual, ambient and target temperature bars
    const totalHeight = max - min;
    const actualHeight = ((localActualTemp - min) / totalHeight) * 100;
    const ambientHeight = ((localAmbientTemp - min) / totalHeight) * 100;
    const targetHeight = ((localTargetTemp - min) / totalHeight) * 100;

    return (
        <Paper>

            <Group>


                <div className="temperature-slider">
                    <div className="temperature-bar range" />

                    <div className="temperature-bar actual" style={{ height: `${actualHeight}%` }} />

                    <div className="temperature-bar ambient" style={{ height: `${ambientHeight}%` }} />
                    <div className="temperature-bar target" style={{ height: `${targetHeight}%` }} />

                </div>
                <Text>Actual: {CtoF(actualTemp)}</Text>
                <Text>Ambient: {CtoF(ambientTemp)}</Text>
                <Text>Target: {CtoF(targetTemp)}</Text>

            </Group>
        </Paper>
    );
};


const CtoF  = (temp: number) => temp * 9 / 5 + 32;