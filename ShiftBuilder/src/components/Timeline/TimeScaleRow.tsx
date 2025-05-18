const TimeScaleRow: React.FC = () => {
    const hours = Array.from({ length: 24 }, (_, i) => i);
    const formatTime = (h: number) => `${String(h).padStart(2, "0")}:00`;
  
    return (
      <div className="flex border-b border-gray-200">
        {hours.map(hour => (
          <div
            key={hour}
            className="w-[60px] h-12 border-r border-gray-200 flex items-center justify-center text-sm text-gray-500"
          >
            {formatTime(hour)}
          </div>
        ))}
      </div>
    );
  };
  
  export default TimeScaleRow;
  