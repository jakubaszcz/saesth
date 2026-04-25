import {getCurrentWindow} from "@tauri-apps/api/window";
import {BoltIcon, X, House, Minus, Maximize2, Minimize2} from "lucide-react";
import {Pages} from "../pages/pages.ts";
import {useEffect, useState} from "react";

type Props = {
  tab: Pages;
  setTab: React.Dispatch<React.SetStateAction<Pages>>;
};

export const Header = ({ tab, setTab }: Props) => {

  const appWindow = getCurrentWindow();

  const [isMaximized, setIsMaximized] = useState(false);

  const handleClose = async () => {
    try {
      await appWindow.close();
    } catch (error) {
      console.error("Failed to close window:", error);
    }
  };

  const handleMinimize = async () => {
    try {
      await appWindow.minimize();
    } catch (error) {
      console.error("Failed to minimize window:", error);
    }
  };

  const handleMaximize = async () => {
    const maximized = await appWindow.isMaximized();

    if (maximized) {
      await appWindow.unmaximize();
    } else {
      await appWindow.maximize();
    }
  };

  useEffect(() => {
    const load = async () => {
      setIsMaximized(await appWindow.isMaximized());
    };

    load();

    const unlistenPromise = appWindow.onResized(async () => {
      setIsMaximized(await appWindow.isMaximized());
    });

    return () => {
      unlistenPromise.then(unlisten => unlisten());
    };
  }, []);

  return (
    <header data-tauri-drag-region className="
    w-full flex justify-between items-center h-14 px-8
    flex items-center justify-between">
      <div className="flex items-center gap-2">
        <div className="
        font-manrope
text-[var(--primary-500)]
text-2xl
tracking-wide
select-none
drop-shadow-sm
">
          Saesth
        </div>
      </div>

      <div className="flex gap-2 text-[var(--primary-500)]">
        { tab === Pages.HOME && (
            <button
                onClick={() => setTab(Pages.SETTINGS)}
                aria-label="Go to settings"
            >
              <BoltIcon size={20} className="transition-all duration-300 ease-out hover:scale-125 hover:text-[var(--primary-600)] cursor-pointer"/>
            </button>
        )}
        { tab === Pages.SETTINGS && (
            <button onClick={() => setTab(Pages.HOME)} aria-label="Go to home">
              <House size={20} className="transition-all duration-300 ease-out hover:scale-125 hover:text-[var(--primary-600)] cursor-pointer"/>
            </button>
        )}
        <button onClick={handleMinimize} aria-label="Minimize">
          <Minus size={20} className="transition-all duration-300 ease-out hover:scale-125 hover:text-[var(--primary-600)] cursor-pointer"/>
        </button>
        {!isMaximized && (
            <button onClick={handleMaximize} aria-label="Minimize">
              <Maximize2 size={20} className="transition-all duration-300 ease-out hover:scale-125 hover:text-[var(--primary-600)] cursor-pointer"/>
            </button>
        )}
        {isMaximized && (
            <button onClick={handleMaximize} aria-label="Minimize">
              <Minimize2 size={20} className="transition-all duration-300 ease-out hover:scale-125 hover:text-[var(--primary-600)] cursor-pointer"/>
            </button>
        )}
        <button onClick={handleClose} aria-label="Close"
        >
          <X size={20} className="transition-all duration-300 ease-out hover:scale-125 hover:text-[var(--primary-600)] cursor-pointer"/>
        </button>
      </div>
    </header>
  );
};
