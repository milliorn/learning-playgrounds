import { Link } from "react-router-dom";
import { UserAuth } from "../context/AuthContext";

function Navbar(): JSX.Element {
  const { user, logOut } = UserAuth();

  async function handleSignOut(): Promise<void> {
    try {
      await logOut();
    } catch (error) {
      console.log(error);
    }
  }

  return (
    <div className="flex justify-between bg-gray-600 w-full p-4">
      <h1 className="text-center text-xl font-bold">Firebase Google Auth</h1>
      {user?.displayName ? (
        <button onClick={handleSignOut}>Logout</button>
      ) : (
        <Link to="/signin">
          <span className="text-white">Sign in</span>
        </Link>
      )}
    </div>
  );
}

export default Navbar;
