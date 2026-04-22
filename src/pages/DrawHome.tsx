import {SoundCard} from "../component/Sound-card.tsx";
import {useEffect, useState} from "react";
import {SoundData} from "../interface/sound-data.ts";
import {invoke} from "@tauri-apps/api/core";

export function DrawHome() {

    const [sounds, setSounds] = useState<SoundData[]>([]);

    useEffect(() => {
        async function fetchSounds() {
            try {
                const fetchedSounds = await invoke<SoundData[]>("get_sounds");
                setSounds(fetchedSounds);
            } catch (error) {
                console.error("Failed loading songs :", error);
            }
        }
        fetchSounds();
    }, []);

    const handleTogglePlay = async (id: string) => {
        try {
            const updatedSounds = await invoke<SoundData[]>("toggle_play", { id });
            setSounds(   updatedSounds);
        } catch (error) {
            console.error("Failed to toggle play:", error);
        }
    };

    const handleVolumeChange = async (id: string, volume: number) => {
        try {
            const updatedSounds = await invoke<SoundData[]>("change_volume", { id, volume });
            setSounds(updatedSounds);
        } catch (error) {
            console.error("Failed to change volume:", error);
        }
    }

    return (
        <div className="max-w-4xl w-full grid grid-cols-1 sm:grid-cols-2 gap-8">
            {sounds.map((data) => (
                <SoundCard
                    key={data.id}
                    id={data.id}
                    data={data}
                    onClick={() => handleTogglePlay(data.id)}
                    onChanged={(volume) => handleVolumeChange(data.id, volume)}
                />
            ))}
        </div>
    )
}