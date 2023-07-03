import React, { useContext } from "react";

import styled from "styled-components";

import { NavbarContext } from "./Navbar";

// Styling for the user container
const UserContainerContainer = styled.div`
  display: flex;
  gap: 1rem;

  p {
    margin-bottom: 0;
  }

  button {
    cursor: pointer;
    display: inline-block;
    font-size: 0.75rem;
    padding: 0.15rem 0.5rem;
    text-transform: capitalize;
  }
`;


const UserContainer = () => {
  const { user, logout } = useContext(NavbarContext);
  // console.log(NavbarContext);

  const formattedName = user && user.name ? user.name.toUpperCase() : "";

  return (
    <UserContainerContainer>
      {user.name !== "Stranger" ? (
        <React.Fragment>
          <p>Hello, {formattedName}!</p>
          <button onClick={logout}>Logout</button>
        </React.Fragment>
      ) : (
        <React.Fragment>
          <p>Hello, {user.name}!</p>
          <button onClick={logout}>Login</button>
        </React.Fragment>
      )}
    </UserContainerContainer>
  );
};

export default UserContainer;
