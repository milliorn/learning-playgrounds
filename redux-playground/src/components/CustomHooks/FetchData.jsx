import React, { useEffect, useState } from "react";
import styled from "styled-components";

// Styling for the container
const Container = styled.div`
  border-top: 1px solid #ccc;
  margin: 2rem 0;
  padding: 2rem 0;
  text-transform: capitalize;
`;

// API endpoint to fetch user data
const url = "https://api.github.com/users/urothis";

// FetchData component
const FetchData = () => {
  // State variables to track loading, error, and user data
  const [isError, setIsError] = useState(false);
  const [isLoading, setIsLoading] = useState(true);
  const [user, setUser] = useState(null);

  // Fetch user data when the component mounts
  useEffect(() => {
    const fetchUser = async () => {
      try {
        const resp = await fetch(url);

        // Check if the response is successful
        if (!resp.ok) {
          setIsError(true);
          setIsLoading(false);
          return;
        }

        // Parse the response as JSON
        const user = await resp.json();
        setUser(user);
      } catch (error) {
        setIsError(true);
        // Handle any errors that occur during the fetch
      }

      // Hide the loading state
      setIsLoading(false);
    };

    fetchUser();
  }, []);
  // The empty dependency array [] ensures that the effect runs only once when the component mounts

  // Render different components based on the loading and error states
  if (isLoading) {
    return <h2>Loading...</h2>;
  }

  if (isError) {
    return <h2>There was an error...</h2>;
  }

  // Destructure the user object
  const { avatar_url, name, company, bio } = user;

  // Render the user information in the container
  return (
    <Container>
      <img
        style={{ width: "100px", borderRadius: "25px" }}
        src={avatar_url}
        alt={name}
      />
      <h2>{name}</h2>
      <h4>works at {company}</h4>
      <p>{bio}</p>
    </Container>
  );
};

export default FetchData;
