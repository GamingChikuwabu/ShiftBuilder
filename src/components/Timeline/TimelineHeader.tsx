import { Clock, Plus } from "lucide-react";

interface Props {
  newMemberName: string;
  setNewMemberName: (val: string) => void;
  onAdd: () => void;
}

const TimelineHeader: React.FC<Props> = ({ newMemberName, setNewMemberName, onAdd }) => (
  <div>
    <div className="timeline-header">
      <Clock className="w-6 h-6 text-blue-600" />
      <h2 className="timeline-title">Timeline View</h2>
    </div>
    <div className="mb-6 flex gap-2">
      <input
        type="text"
        value={newMemberName}
        onChange={(e) => setNewMemberName(e.target.value)}
        placeholder="Enter member name"
        className="member-input"
      />
      <button
        onClick={onAdd}
        className="flex items-center gap-2 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
      >
        <Plus className="w-4 h-4" />
        Add Member
      </button>
    </div>
  </div>
);

export default TimelineHeader;
