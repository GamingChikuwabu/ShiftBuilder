import { invoke } from "@tauri-apps/api/core";
import React, { useState } from "react";

const HomePage = () => {
  const [date, setDate] = useState("");
  const [time, setTime] = useState("");

  const handleCreateSheet = async () => {
    const [yyyy, mm, dd] = date.split("-");
    const [hh, min] = time.split(":");
    const sheetName = `${yyyy}_${mm}_${dd}_${hh}_${min}`;

    await invoke("create_shift_sheet", { sheetName });
    alert(`シフトシート ${sheetName} を作成しました`);
  };

  return (
    <div className="p-4">
      <h1 className="text-2xl font-bold mb-4">🏠 Home Page</h1>

      <div className="space-y-2">
        <label>
          日付：
          <input type="date" value={date} onChange={(e) => setDate(e.target.value)} />
        </label>
        <br />
        <label>
          時間：
          <input type="time" value={time} onChange={(e) => setTime(e.target.value)} />
        </label>
        <br />
        <button
          className="mt-2 px-4 py-2 bg-blue-600 text-white rounded"
          onClick={handleCreateSheet}
        >
          新規シフト作成
        </button>
      </div>
    </div>
  );
};

export default HomePage;
