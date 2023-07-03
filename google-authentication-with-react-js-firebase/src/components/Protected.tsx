import { Navigate } from "react-router-dom";
import { UserAuth } from "../context/AuthContext";

function Protected({ children }: any): any {
  const { user } = UserAuth();
  
  if (!user) {
    return <Navigate to="/" />;
  }

  return children;
}

export default Protected;
