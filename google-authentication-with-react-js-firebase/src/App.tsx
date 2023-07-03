import Account from "./pages/Account";
import Home from "./pages/Home";
import Navbar from "./components/Navbar";
import Protected from "./components/Protected";
import Signin from "./pages/Signin";
import { AuthContextProvider } from "./context/AuthContext";
import { Route, Routes } from "react-router-dom";

function App(): JSX.Element {
  return (
    <div>
      <AuthContextProvider>
        <Navbar />
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/signin" element={<Signin />} />
          <Route
            path="/account"
            element={
              <Protected>
                <Account />
              </Protected>
            }
          />
        </Routes>
      </AuthContextProvider>
    </div>
  );
}

export default App;
