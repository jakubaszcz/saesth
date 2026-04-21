import { getCurrentWindow } from "@tauri-apps/api/window";
import { X } from "lucide-react";

export const Header = () => {
  const appWindow = getCurrentWindow();

  const handleClose = async () => {
    try {
      await appWindow.close();
    } catch (error) {
      console.error("Failed to close window:", error);
    }
  };

  return (
    <header 
      data-tauri-drag-region 
      className="flex items-center justify-between px-6 py-2 bg-linear-to-r from-[var(--background-700)] to-[var(--background-900)] text-white select-none h-14 border-b border-white/5 cursor-default"
    >
      <div className="font-bold text-xl tracking-wider pointer-events-none opacity-80 select-none">
        Saesth
      </div>

      <div className="flex items-center relative z-10">
        <button 
          onClick={handleClose}
          className="hover:bg-red-500/20 hover:text-red-500 rounded-full p-1.5 transition-all cursor-pointer group"
          aria-label="Fermer"
        >
          <X size={20} className="group-hover:scale-110 transition-transform" />
        </button>
      </div>
    </header>
  );
};
