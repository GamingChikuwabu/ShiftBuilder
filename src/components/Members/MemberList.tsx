import { ShiftEntry } from "../../types/ShiftDataTypes";

interface Props {
  entries: ShiftEntry[];
  onRemove: (id: string) => void;
}

export const MemberList: React.FC<Props> = ({ entries, onRemove }) => (
  <ul>
    {entries.map(entry => (
      <li key={entry.id}>
        {entry.name}
        <button onClick={() => onRemove(entry.id)}>削除</button>
      </li>
    ))}
  </ul>
);
