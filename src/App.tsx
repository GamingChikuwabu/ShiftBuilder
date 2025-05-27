import { useEffect, useState } from "react";
import { Routes, Route, Navigate } from "react-router-dom";
import Header from './components/Header/Header';
import HomePage from "./pages/HomePage";
import ShiftPage from "./pages/ShiftPage";
import MemberPage from "./pages/MemberPage";
import LoginPage from "./pages/LoginPage"; // 追加

function App() {
  const [isLoggedIn, setIsLoggedIn] = useState<boolean | null>(null);

  // 起動時にログイン状態をチェック（初期値：falseでもOK）
  useEffect(() => {
    // 本来はRust側にログイン状態を問い合わせる
    setIsLoggedIn(false); // 仮に未ログインから始める
  }, []);

  if (isLoggedIn === null) return <div>Loading...</div>;

  return (
    <div className="min-h-screen bg-gray-100">
      {isLoggedIn && <Header />}
      <Routes>
        {isLoggedIn ? (
          <>
            <Route path="/" element={<HomePage />} />
            <Route path="/shift" element={<ShiftPage />} />
            <Route path="/member" element={<MemberPage />} />
            <Route path="*" element={<Navigate to="/" />} />
          </>
        ) : (
          <>
            <Route path="/login" element={<LoginPage onLogin={() => setIsLoggedIn(true)} />} />
            <Route path="*" element={<Navigate to="/login" />} />
          </>
        )}
      </Routes>
    </div>
  );
}

export default App;
