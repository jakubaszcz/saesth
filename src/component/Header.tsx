import { getCurrentWindow } from "@tauri-apps/api/window";
import { X, Minimize } from "lucide-react";
import logo from "../assets/saesth.svg";
function Logo({ size = 32 }) {
  return <img src={logo} width={size} height={size} />;
}
export const Header = () => {
  const appWindow = getCurrentWindow();

  const handleClose = async () => {
    console.log("close")

    try {
      await appWindow.close();
    } catch (error) {
      console.error("Failed to close window:", error);
    }
  };

  const handleMinimize = async () => {
    console.log("minimize")
    try {
      await appWindow.minimize();
    } catch (error) {
      console.error("Failed to minimize window:", error);
    }
  };



  return (
    <header 
      data-tauri-drag-region 
      className="flex items-center justify-between px-6 py-2 bg-linear-to-r from-[var(--background-700)] to-[var(--background-900)] text-white select-none h-14 border-b border-white/5 cursor-default"
    >
      <div className="flex items-center gap-3">
        <div className="opacity-80">
          <Logo />
        </div>

        <div className="font-bold text-xl tracking-wider pointer-events-none opacity-80 select-none">
          Saesth
        </div>
      </div>

      <div className="flex items-end relative z-10">
        <button
            onClick={handleMinimize}
            className="hover:bg-[var(--minimize-secondary)] hover:text-[var(--minimize-primary)] rounded-full p-1.5 transition-all cursor-pointer group"
            aria-label="Minimize"
        >
          <Minimize size={20} className="group-hover:scale-110 transition-transform" />
        </button>
        <button 
          onClick={handleClose}
          className="hover:bg-[var(--close-secondary)] hover:text-[var(--close-primary)] rounded-full p-1.5 transition-all cursor-pointer group"
          aria-label="Close"
        >
          <X size={20} className="group-hover:scale-110 transition-transform" />
        </button>
      </div>
    </header>
  );
};
