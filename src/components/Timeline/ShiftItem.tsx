import React from "react";
import { GripHorizontal, X } from "lucide-react";
import { ShiftEntry } from "../../types/ShiftDataTypes";

interface Props {
  shift: ShiftEntry;
  onDragStart: (e: React.MouseEvent, shift: ShiftEntry) => void;
  onResizeStart: (e: React.MouseEvent, shift: ShiftEntry, edge: 'start' | 'end') => void;
  onRemove: (id: string) => void;
  getPositionFromTime: (time: string) => number;
}

const ShiftItem: React.FC<Props> = ({
  shift,
  onDragStart,
  onResizeStart,
  onRemove,
  getPositionFromTime,
}) => {
  const left = getPositionFromTime(shift.start);
  const width = getPositionFromTime(shift.end) - left;

  return (
    <div
      className="shift-block"
      style={{ left: `${left}px`, width: `${width}px` }}
      onMouseDown={(e) => onDragStart(e, shift)}
    >
      <div
        className="shift-handle shift-handle-left"
        onMouseDown={(e) => onResizeStart(e, shift, 'start')}
      >
        <GripHorizontal className="w-4 h-4 text-white" />
      </div>
      <span className="shift-time">{shift.start} - {shift.end}</span>
      <div className="shift-actions">
        <button onClick={(e) => { e.stopPropagation(); onRemove(shift.id); }}>
          <X className="w-4 h-4 text-white" />
        </button>
        <div
          className="shift-handle shift-handle-right"
          onMouseDown={(e) => onResizeStart(e, shift, 'end')}
        >
          <GripHorizontal className="w-4 h-4 text-white" />
        </div>
      </div>
    </div>
  );
};

export default ShiftItem;
