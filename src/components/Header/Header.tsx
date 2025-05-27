import React from "react";
import { Link } from "react-router-dom";

const Header: React.FC = () => {
  return (
    <header className="bg-blue-600 text-white">
      <div className="p-4 text-2xl font-bold">ShiftBuilder</div>
      <nav className="bg-blue-500 px-4 py-2">
        <ul className="flex space-x-6 text-lg">
        <li><Link to="/" className="hover:underline">Home</Link></li>
        <li><Link to="/shift" className="hover:underline">Shift</Link></li>
        <li><Link to="/member" className="hover:underline">Member</Link></li>
        </ul>
      </nav>
    </header>
  );
};

export default Header;