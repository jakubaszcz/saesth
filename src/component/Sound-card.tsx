import { SoundData } from "../interface/sound-data.ts";
import { CloudRainWind, CirclePlay, CirclePauseIcon, FlameIcon, Bird, Wind, Volume2 } from "lucide-react"


interface SoundCardProps {
    id: string;
    data: SoundData;
    onClick: () => void;
    onChanged?: (volume: number) => void;
}

const getIcon = (id: string) => {
    const nid = id.toLowerCase();
    if (nid.includes("rain")) return <CloudRainWind className="w-8 h-8 mb-2" />;
    if (nid.includes("fire")) return <FlameIcon className="w-8 h-8 mb-2" />;
    if (nid.includes("bird")) return <Bird className="w-8 h-8 mb-2" />;
    if (nid.includes("wind")) return <Wind className="w-8 h-8 mb-2" />;
    return <Volume2 className="w-8 h-8 mb-2" />;
}

export const SoundCard = ({id,  data, onClick, onChanged} : SoundCardProps) => {
    const nid = id.toLowerCase();

    return (
        <div 
            className="p-6 rounded-2xl shadow-lg transition-all duration-300 hover:scale-[1.02] flex flex-col items-center text-center border border-white/10"
            style={{
                backgroundImage: `linear-gradient(135deg,
                    var(--${nid}-900),
                    var(--${nid}-500)
                )`
            }}
        >
            <div style={{ color: `var(--${nid}-100)` }}>
                {getIcon(id)}
            </div>
            
            <h3 className="font-bold text-xl capitalize mb-4"
                style={{
                    color: `var(--${nid}-100)`,
                }}
            >{id}</h3>

            <button
                onClick={onClick}
                className="mb-6 px-6 py-2 rounded-full font-semibold flex items-center gap-2 transition-transform active:scale-95 shadow-md"
                style={{
                    background: `var(--${nid}-100)`,
                    color: `var(--${nid}-600)`,
                }}
            >
                {data.play ? <CirclePauseIcon size={20} /> : <CirclePlay size={20} />}
                {data.play ? "Pause" : "Play"}
            </button>

            <div className="w-full space-y-2 mt-auto">
                <div className="flex justify-between text-xs font-medium uppercase tracking-wider"
                     style={{ color: `var(--${nid}-200)` }}>
                    <span>Volume</span>
                    <span>{Math.round(data.volume * 100)}%</span>
                </div>
                <input
                    type="range"
                    min={0}
                    max={100}
                    step={1}
                    value={data.volume * 100}
                    onChange={(e) => onChanged?.(parseFloat(e.target.value) / 100)}
                    className="w-full h-1.5 bg-black/20 rounded-lg appearance-none cursor-pointer accent-white"
                />
            </div>
        </div>
    )
}