import React from "react";
import styled from "styled-components";

// Styling for the navbar links container
const NavbarLinksContainer = styled.div`
  ul {
    display: flex;
    gap: 1rem;
    list-style: none;
    margin: 0;
    padding: 0;
  }

  a {
    color: #fff;
    cursor: pointer;
    text-transform: capitalize;
  }
`;

const NavbarLinks = () => {
  return (
    <NavbarLinksContainer>
      <ul>
        <li>
          <a href="#">home</a>
        </li>
        <li>
          <a href="#">about</a>
        </li>
      </ul>
    </NavbarLinksContainer>
  );
};

export default NavbarLinks;
