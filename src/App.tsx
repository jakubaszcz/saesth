import "./App.css";
import {Header} from "./component/Header.tsx";
import { DrawHome } from "./pages/DrawHome.tsx";
import {useState} from "react";
import {Pages} from "./pages/pages.ts";
function App() {

  const [tab, setTab] = useState<Pages>(Pages.HOME);

  return (
    <div className="flex flex-col h-screen w-screen overflow-hidden">
      <Header tab={tab} setTab={setTab}/>
      <main className="
      flex-1
      overflow-y-auto
      bg-linear-to-r from-[var(--background-700)] to-[var(--background-900)] p-8 pt-4
      flex items-center justify-center">
        {tab === Pages.HOME && <DrawHome />}
      </main>
    </div>
  );
}

export default App;
