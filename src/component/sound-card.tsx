import { SoundData } from "../interface/sound-data.ts";

interface SoundCardProps {
    id: string;
    data: SoundData;
    onClick: () => void;
}

export const SoundCard = ({id,  data, onClick} : SoundCardProps) => {
    return (
        <div>
            <h3>{id}</h3>
            <p>Path : {data.path}</p>
            <p>Play : {data.play ? "Yes" : "No"}</p>
            <button onClick={onClick}>Toggle Play</button>
        </div>
    )
}