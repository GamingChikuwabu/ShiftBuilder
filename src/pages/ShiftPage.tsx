import TimeLineView from '../components/Timeline/TimelineView';
import { useState } from 'react';
import { ShiftEntry } from '../types/ShiftDataTypes';




const ShiftPage = () => {
    const [shifts, setShifts] = useState<ShiftEntry[]>([]);
    const handleAddShift = (name: string) => {
        const newShift: ShiftEntry = {
          id: `shift-${Date.now()}`,
          name,
          start: '09:00',
          end: '17:00',
        };
        setShifts([...shifts, newShift]);
      };
    
      const handleUpdateShift = (updatedShift: ShiftEntry) => {
        setShifts(currentShifts =>
          currentShifts.map(shift =>
            shift.id === updatedShift.id ? updatedShift : shift
          )
        );
      };
    
      const handleRemoveShift = (shiftId: string) => {
        setShifts(currentShifts => currentShifts.filter(shift => shift.id !== shiftId));
      };
    return (
        <TimeLineView
        shifts={shifts}
        onAddShift={handleAddShift}
        onUpdateShift={handleUpdateShift}
        onRemoveShift={handleRemoveShift}
      />
    );
  };
  
  export default ShiftPage;
  