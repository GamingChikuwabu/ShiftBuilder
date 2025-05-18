import { useState } from "react";
import { ShiftEntry } from "../types/Shift";

export function useShiftData() {
  const [entries, setEntries] = useState<ShiftEntry[]>([]);

  const updateShift = (id: string, newStart: string, newEnd: string) => {
    setEntries(prev =>
      prev.map(e => (e.id === id ? { ...e, start: newStart, end: newEnd } : e))
    );
  };

  return { entries, setEntries, updateShift };
}
