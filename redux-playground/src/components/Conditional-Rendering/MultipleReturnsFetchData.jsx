import React, { useState, useEffect } from "react";

const url = "https://api.github.com/users/milliorn";

const MultipleReturnsFetchData = () => {
  const [error, setError] = useState(false);
  const [loading, setLoading] = useState(true);
  const [user, setUser] = useState(null);

  useEffect(() => {
    // This useEffect hook is used to fetch user data from the API and update the state

    const fetchUser = async () => {
      try {
        const response = await fetch(url); // Make a request to the API
        const user = await response.json(); // Convert the response to JSON format
        console.log(user); // Log the user data to the console
        setUser(user); // Update the user state with the fetched user data
      } catch (error) {
        setError(true); // If there's an error during the API request, set the error state to true
        console.log(error); // Log the error to the console
      } finally {
        setLoading(false); // Set the loading state to false, indicating that the data has been fetched
      }
    };

    const timeout = setTimeout(() => {
      fetchUser(); // Call the fetchUser function after a delay of 1000 milliseconds (1 second)
    }, 1000);

    return () => {
      clearTimeout(timeout); // Clean up the timeout when the component unmounts or the dependencies change
    };
  }, []);

  if (loading) {
    // If the loading state is true, display a loading message
    return <h2>Loading...</h2>;
  }

  if (error) {
    // If the error state is true, display an error message
    return <h2>Error...</h2>;
  }

  const formatTime = (time) => {
    // A helper function to format the user's created_at time
    const created = new Date(time);
    const formatted = created.toLocaleDateString("en-US");
    return formatted;
  };

  return (
    // Render the user data if there are no loading or error states
    <div>
      <div>
        <h2>{user.login}</h2>
        <img src={user.avatar_url} alt={user.login} />
      </div>
      <div>
        <span>
          Blog:{" "}
          <a target="_blank" href={user.blog} style={{ color: "lightblue" }}>
            Link
          </a>
        </span>
        {" | "}
        <span>
          API:{" "}
          <a target="_blank" href={user.url} style={{ color: "lightblue" }}>
            Link
          </a>
        </span>
      </div>
      <div>
        <span>Created: {formatTime(user.created_at)}</span>
      </div>
      <div className="">
        <span>Location: {user.location}</span>
      </div>
      <div>
        <span>Followers: {user.followers}</span>
        {" | "}
        <span>Following: {user.following}</span>
      </div>
      <div>
        <span>Public Repos: {user.public_repos}</span>
        {" | "}
        <span>Public Gists: {user.public_gists}</span>
      </div>
    </div>
  );
};

export default MultipleReturnsFetchData;
