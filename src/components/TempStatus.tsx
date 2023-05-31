import { RingProgress } from "@mantine/core";
import { BedJetStatus } from "../hooks";

type TempSegment = {
    value: number,
    color: string,
}

function calculatePercentage(status: BedJetStatus): TempSegment[] {
    const min = status.min_target_temp;
    const max = status.max_target_temp;
    
    // if max and min are equal, return array with zero percentages
    if (max === min) {
        return [
            { value: 0, color: 'red' },
            { value: 0, color: 'green' },
            { value: 0, color: 'blue' },
        ]
    }
    
    const ambientPercentage = ((status.ambient_temp - min) / (max - min)) * 100;
    const actualPercentage = ((status.actual_temp - min) / (max - min)) * 100;
    const targetPercentage = ((status.target_temp - min) / (max - min)) * 100;

    return [
        { value: ambientPercentage, color: 'red' },
        { value: actualPercentage, color: 'green' },
        { value: targetPercentage, color: 'blue' },
    ]
}


interface TempStatusProps {
    status: BedJetStatus
}
export function TempStatus({ status }: TempStatusProps) {

    const sections = calculatePercentage(status);

    return (
        <RingProgress sections={sections}>

        </RingProgress>
    )
}