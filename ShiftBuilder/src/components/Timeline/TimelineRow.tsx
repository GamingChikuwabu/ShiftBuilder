import { ShiftEntry } from "../../types/Shift";
import ShiftItem from "./ShiftItem";

interface Props {
  shift: ShiftEntry;
  onDragStart: any;
  onResizeStart: any;
  onRemove: any;
  getPositionFromTime: (time: string) => number;
}

const TimelineRow: React.FC<Props> = ({ shift, ...rest }) => (
  <div className="flex">
    <div className="member-column">
      <div className="member-cell">{shift.name}</div>
    </div>
    <div className="flex-1 relative h-10 border-b border-gray-100">
      <ShiftItem shift={shift} {...rest} />
    </div>
  </div>
);

export default TimelineRow;
