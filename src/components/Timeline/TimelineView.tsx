import React, { useState, useRef } from 'react';
import { ShiftEntry } from '../../types/Shift';
import TimelineHeader from "./TimelineHeader";
import TimeScaleRow from "./TimeScaleRow";
import TimelineRow from "./TimelineRow";
import './styles.css';

interface TimeLineViewProps {
  shifts: ShiftEntry[];
  onAddShift: (name: string) => void;
  onUpdateShift: (shift: ShiftEntry) => void;
  onRemoveShift: (shiftId: string) => void;
}

const TimeLineView: React.FC<TimeLineViewProps> = ({
  shifts,
  onAddShift,
  onUpdateShift,
  onRemoveShift,
}) => {
  const [isDragging, setIsDragging] = useState(false);
  const [isResizing, setIsResizing] = useState(false);
  const [newMemberName, setNewMemberName] = useState('');
  const draggedShift = useRef<{ id: string; startX: number; originalStart: string; originalEnd: string } | null>(null);
  const resizingShift = useRef<{ id: string; startX: number; originalStart: string; originalEnd: string; edge: 'start' | 'end' } | null>(null);

  

  const formatTime = (hour: number, minute: number): string => {
    return `${hour.toString().padStart(2, '0')}:${minute.toString().padStart(2, '0')}`;
  };

  const handleDragStart = (e: React.MouseEvent, shift: ShiftEntry) => {
    e.stopPropagation();
    setIsDragging(true);
    draggedShift.current = {
      id: shift.id,
      startX: e.clientX,
      originalStart: shift.start,
      originalEnd: shift.end
    };
  };

  const handleResizeStart = (e: React.MouseEvent, shift: ShiftEntry, edge: 'start' | 'end') => {
    e.stopPropagation();
    setIsResizing(true);
    resizingShift.current = {
      id: shift.id,
      startX: e.clientX,
      originalStart: shift.start,
      originalEnd: shift.end,
      edge
    };
  };

  const handleDragMove = (e: React.MouseEvent) => {
    if (isDragging && draggedShift.current) {
      const deltaX = e.clientX - draggedShift.current.startX;
      const minutesPerPixel = 1;
      const timeShift = Math.round(deltaX * minutesPerPixel);

      const [originalStartHour, originalStartMinute] = draggedShift.current.originalStart.split(':').map(Number);
      const [originalEndHour, originalEndMinute] = draggedShift.current.originalEnd.split(':').map(Number);

      const startMinutes = originalStartHour * 60 + originalStartMinute + timeShift;
      const endMinutes = originalEndHour * 60 + originalEndMinute + timeShift;

      if (startMinutes < 0 || endMinutes > 24 * 60) return;

      const newStartHour = Math.floor(startMinutes / 60);
      const newStartMinute = startMinutes % 60;
      const newEndHour = Math.floor(endMinutes / 60);
      const newEndMinute = endMinutes % 60;

      const shift = shifts.find(s => s.id === draggedShift.current?.id);
      if (shift) {
        onUpdateShift({
          ...shift,
          start: formatTime(newStartHour, newStartMinute),
          end: formatTime(newEndHour, newEndMinute),
        });
      }
    } else if (isResizing && resizingShift.current) {
      const deltaX = e.clientX - resizingShift.current.startX;
      const minutesPerPixel = 1;
      const timeShift = Math.round(deltaX * minutesPerPixel);

      const [originalStartHour, originalStartMinute] = resizingShift.current.originalStart.split(':').map(Number);
      const [originalEndHour, originalEndMinute] = resizingShift.current.originalEnd.split(':').map(Number);

      const shift = shifts.find(s => s.id === resizingShift.current?.id);
      if (shift) {
        if (resizingShift.current.edge === 'start') {
          const newStartMinutes = originalStartHour * 60 + originalStartMinute + timeShift;
          const endMinutes = originalEndHour * 60 + originalEndMinute;
          
          if (newStartMinutes < 0 || newStartMinutes >= endMinutes - 30) return;

          onUpdateShift({
            ...shift,
            start: formatTime(Math.floor(newStartMinutes / 60), newStartMinutes % 60),
          });
        } else {
          const startMinutes = originalStartHour * 60 + originalStartMinute;
          const newEndMinutes = originalEndHour * 60 + originalEndMinute + timeShift;
          
          if (newEndMinutes > 24 * 60 || newEndMinutes <= startMinutes + 30) return;

          onUpdateShift({
            ...shift,
            end: formatTime(Math.floor(newEndMinutes / 60), newEndMinutes % 60),
          });
        }
      }
    }
  };

  const handleDragEnd = () => {
    setIsDragging(false);
    setIsResizing(false);
    draggedShift.current = null;
    resizingShift.current = null;
  };

  const addNewMember = () => {
    if (!newMemberName.trim()) return;
    onAddShift(newMemberName);
    setNewMemberName('');
  };

  const getPositionFromTime = (time: string): number => {
    const [hours, minutes] = time.split(':').map(Number);
    return hours * 60 + minutes;
  };

  return (
    <div className="timeline-container">
      <TimelineHeader
        newMemberName={newMemberName}
        setNewMemberName={setNewMemberName}
        onAdd={addNewMember}
      />
      <div className="timeline-grid">
        <div className="timeline-content">
          <div className="flex">
            <div className="member-column">
              <div className="member-header">Members</div>
            </div>
            <div className="flex-1">
              <TimeScaleRow />
            </div>
          </div>
          <div 
            className="relative" 
            onMouseMove={handleDragMove}
            onMouseUp={handleDragEnd}
            onMouseLeave={handleDragEnd}
          >
            {shifts.map(shift => (
              <TimelineRow
                key={shift.id}
                shift={shift}
                onDragStart={handleDragStart}
                onResizeStart={handleResizeStart}
                onRemove={onRemoveShift}
                getPositionFromTime={getPositionFromTime}
              />
            ))}
          </div>
        </div>
      </div>
    </div>
  );
};

export default TimeLineView;