import { invoke } from "@tauri-apps/api/core";

type Props = {
  onLogin: () => void;
};

const LoginPage: React.FC<Props> = ({ onLogin }) => {
  const openGoogleLoginInBrowser = () => {
    const clientId = "260763329720-s35qh245gtkoidbc27ff9as4c47pbq4s.apps.googleusercontent.com";
    const redirectUri = "http://localhost:8080";
  
    const scope = [
      "https://www.googleapis.com/auth/userinfo.email",
      "https://www.googleapis.com/auth/spreadsheets",
      "https://www.googleapis.com/auth/drive.file" ,// ✅ Driveファイル作成に必要
      "https://www.googleapis.com/auth/drive.appdata"
    ].join(" ");
  
    const url = `https://accounts.google.com/o/oauth2/v2/auth` +
                `?response_type=code` +
                `&client_id=${clientId}` +
                `&redirect_uri=${redirectUri}` +
                `&scope=${encodeURIComponent(scope)}` +
                `&access_type=offline`;
  
    console.log("認証URL:", url);
    window.open(url, "_blank");
  };
  

  const handleLogin = async () => {
    try {
      const clientId = import.meta.env.VITE_GOOGLE_CLIENT_ID;
      const clientSecret = import.meta.env.VITE_GOOGLE_CLIENT_SECRET;
      const redirectUri = import.meta.env.VITE_GOOGLE_REDIRECT_URI;

      console.log("ログイン処理を開始します...");
      // サーバー起動成功後にブラウザを開く
      openGoogleLoginInBrowser();
      // 先にサーバーを起動
      const email = await invoke<string>("start_google_login", {
        params: {
          client_id: clientId,
          client_secret: clientSecret,
          redirect_uri: redirectUri
        }
      });
      console.log("ログイン成功:", email);
      onLogin();
    } catch (error) {
      console.error("ログイン失敗:", error);
      alert(`ログインに失敗しました: ${error}`);
    }
  };

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-100">
      <div className="bg-white p-8 rounded-lg shadow-md">
        <h1 className="text-2xl font-bold mb-6 text-center">ShiftBuilder</h1>
        <button
          className="w-full bg-blue-600 hover:bg-blue-700 text-white px-6 py-3 rounded-md transition-colors duration-200 flex items-center justify-center"
          onClick={handleLogin}
        >
          <svg className="w-5 h-5 mr-2" viewBox="0 0 24 24">
            <path
              fill="currentColor"
              d="M12.545,10.239v3.821h5.445c-0.712,2.315-2.647,3.972-5.445,3.972c-3.332,0-6.033-2.701-6.033-6.032s2.701-6.032,6.033-6.032c1.498,0,2.866,0.549,3.921,1.453l2.814-2.814C17.503,2.988,15.139,2,12.545,2C7.021,2,2.543,6.477,2.543,12s4.478,10,10.002,10c8.396,0,10.249-7.85,9.426-11.748L12.545,10.239z"
            />
          </svg>
          Googleでログイン
        </button>
      </div>
    </div>
  );
};

export default LoginPage;
