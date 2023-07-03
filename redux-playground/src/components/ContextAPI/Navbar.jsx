import React, { useState, createContext } from "react";

import styled from "styled-components";

import NavbarLinks from "./NavbarLinks";
import UserContainer from "./UserContainer";

// Styling for the navbar container
const NavbarContainer = styled.div`
  background: #333;
  color: #fff;
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem;
`;

// Styling for the logo
const Logo = styled.h5`
  font-weight: bold;
  margin: 0;
`;

// Styling for the links container
const LinksContainer = styled.div`
  display: flex;
  align-items: center;
`;

// Styling for the link
const Link = styled.a`
  color: #fff;
  cursor: pointer;
  text-transform: capitalize;
  margin-right: 1rem;
`;

export const NavbarContext = createContext();
console.log(NavbarContext);

const Navbar = () => {
  const [user, setUser] = useState({ name: "Stranger" });

  const logout = () => {
    setUser(null);
  };

  return (
    <NavbarContext.Provider value={{ user, setUser }}>
      <NavbarContainer>
        <Logo>CONTEXT API</Logo>
        <LinksContainer>
          <NavbarLinks />
        </LinksContainer>
        <UserContainer user={user} logout={logout} />
      </NavbarContainer>
    </NavbarContext.Provider>
  );
};

export default Navbar;
