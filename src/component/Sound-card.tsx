import { SoundData } from "../interface/sound-data.ts";
import {
    CloudRainWind,
    Play,
    FlameIcon,
    Bird,
    Wind,
    Volume2, Pause
} from "lucide-react";

interface SoundCardProps {
    id: string;
    data: SoundData;
    onClick: () => void;
    onChanged?: (volume: number) => void;
}

const getIcon = (id: string) => {
    const nid = id.toLowerCase();

    if (nid.includes("rain")) return <CloudRainWind size={24} />;
    if (nid.includes("fire")) return <FlameIcon size={24} />;
    if (nid.includes("fire")) return <Bird size={24} />;
    if (nid.includes("wind")) return <Wind size={24} />;

    return <Volume2 size={24} />;
};

export const SoundCard = ({
                              id,
                              data,
                              onClick,
                              onChanged
                          }: SoundCardProps) => {
    return (
        <div
            className="
      rounded-lg
      bg-white/5
      backdrop-blur-md
      border border-white/10
      shadow-[0_10px_40px_rgba(0,0,0,0.18)]
      p-5
      flex flex-col gap-5
      transition-all duration-300
      hover:bg-white/[0.07]
      hover:shadow-[0_14px_44px_rgba(0,0,0,0.24)]
    "
        >
            <div className="flex items-start justify-between gap-4">
                <div className="flex items-center gap-3">
                    <div className="text-[var(--primary-100)]">
                        {getIcon(id)}
                    </div>

                    <div className="flex flex-col">
                        <h3 className="text-[var(--primary-100)] font-semibold text-xl capitalize">
                            {id}
                        </h3>
                    </div>
                </div>

                <button
                    onClick={onClick}
                    className="
          w-11 h-11
          rounded-xl
          flex items-center justify-center
          bg-white/10
          border border-white/10
          text-[var(--primary-100)]
          transition-all duration-300
          hover:bg-white/20
          hover:scale-105
          active:scale-95
        "
                >
                    {data.play ? (
                        <Pause size={20} />
                    ) : (
                        <Play size={20} />
                    )}
                </button>
            </div>

            <div className="flex flex-col gap-2">
                <div className="flex items-center justify-between text-sm uppercase tracking-wide text-[var(--primary-100)]">
                    <span>Volume</span>
                    <span>{Math.round(data.volume * 100)}%</span>
                </div>

                <input
                    type="range"
                    min={0}
                    max={100}
                    step={1}
                    value={data.volume * 100}
                    onChange={(e) =>
                        onChanged?.(parseFloat(e.target.value) / 100)
                    }
                    className="
    w-full
    h-2
    appearance-none
    rounded-full
    bg-white/10
    cursor-pointer

    [&::-webkit-slider-runnable-track]:h-2
    [&::-webkit-slider-runnable-track]:rounded-full
    [&::-webkit-slider-runnable-track]:bg-white/10

    [&::-webkit-slider-thumb]:appearance-none
    [&::-webkit-slider-thumb]:mt-[-4px]
    [&::-webkit-slider-thumb]:h-4
    [&::-webkit-slider-thumb]:w-4
    [&::-webkit-slider-thumb]:rounded-full
    [&::-webkit-slider-thumb]:bg-white/90
    [&::-webkit-slider-thumb]:shadow-md
    [&::-webkit-slider-thumb]:transition-all
    [&::-webkit-slider-thumb]:duration-200
    hover:[&::-webkit-slider-thumb]:scale-110

    [&::-moz-range-track]:h-2
    [&::-moz-range-track]:rounded-full
    [&::-moz-range-track]:bg-white/10

    [&::-moz-range-thumb]:h-4
    [&::-moz-range-thumb]:w-4
    [&::-moz-range-thumb]:rounded-full
    [&::-moz-range-thumb]:border-0
    [&::-moz-range-thumb]:bg-[var(--primary-100)]
  "
                />
            </div>
        </div>
    );
};