import { UserAuth } from "../context/AuthContext";

function Account(): JSX.Element {
  const { logOut, user } = UserAuth();

  async function handleSignOut() {
    try {
      await logOut();
    } catch (error) {
      console.log(error);
    }
  }

  return (
    <div className="w-72 m-auto">
      <h1 className="text-center text-2xl font-bold pt-12">Account</h1>
      <div>
        <p className="text-center my-4 mx-auto">Hello, {user?.displayName}</p>
      </div>
      <button
        onClick={handleSignOut}
        className="border-2 p-4 my-8 flex justify-center mx-auto border-black	"
      >
        Logout
      </button>
    </div>
  );
}

export default Account;
