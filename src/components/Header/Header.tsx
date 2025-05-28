import React, { useState, useEffect } from "react";
import { Link } from "react-router-dom";
import { invoke } from "@tauri-apps/api/core";

const Header: React.FC = () => {
  const [isMenuOpen, setIsMenuOpen] = useState(false);
  const [userEmail, setUserEmail] = useState<string>("");

  useEffect(() => {
    const fetchUserEmail = async () => {
      try {
        const email = await invoke<string>("get_current_user");
        setUserEmail(email);
      } catch (error) {
        console.error("ユーザー情報の取得に失敗:", error);
      }
    };
    fetchUserEmail();
  }, []);

  const handleLogout = async () => {
    try {
      await invoke("logout");
      setIsMenuOpen(false);
      window.location.href = "/login";
    } catch (error) {
      console.error("ログアウトに失敗しました:", error);
      alert("ログアウトに失敗しました。もう一度お試しください。");
    }
  };

  return (
    <header className="bg-blue-600 text-white">
      <div className="flex justify-between items-center p-4">
        <div className="text-2xl font-bold">ShiftBuilder</div>
        <div className="relative">
          <button
            onClick={() => setIsMenuOpen(!isMenuOpen)}
            className="flex items-center space-x-2 hover:bg-blue-500 p-2 rounded"
          >
            <div className="flex items-center space-x-2">
              <span className="text-sm">{userEmail}</span>
              <svg
                xmlns="http://www.w3.org/2000/svg"
                className="h-6 w-6"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
                />
              </svg>
            </div>
          </button>
          {isMenuOpen && (
            <div className="absolute right-0 mt-2 w-48 bg-white rounded-md shadow-lg py-1 z-10">
              <div className="px-4 py-2 text-sm text-gray-700 border-b">
                {userEmail}
              </div>
              <button
                onClick={handleLogout}
                className="block w-full text-left px-4 py-2 text-sm text-gray-700 hover:bg-gray-100"
              >
                ログアウト
              </button>
            </div>
          )}
        </div>
      </div>
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