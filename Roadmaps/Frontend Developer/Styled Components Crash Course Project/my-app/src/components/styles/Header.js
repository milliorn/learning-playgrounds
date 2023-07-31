import React from "react";
import { StyledHeader, Nav, Logo, Image } from "./Header.styled";
import { Container } from "./Container.styled";
import { Button } from "./Button.styled";
import { Flex } from "./Flex.styled";

const Header = () => {
  return (
    <StyledHeader>
      <Container>
        <Nav>
          <Logo src="./images/logo.svg" alt="" />
          <Button>Try It For Free</Button>
        </Nav>
        <Flex>
          <div>
            <h1>Build The Community Your Fans Will Love</h1>
            <p>
              What is Huddle? Re-imagine how your organization stores, shares
              and collaborates on content internally and externally with
              customers, partners and suppliers. Used by 100,000 businesses and
              governments worldwide, Huddle is a proud provider of cloud content
              collaboration for the enterprise. We help our customers store,
              share and collaborate on content from anywhere, on any device,
              inside and between organizations.
            </p>
            <Button bg="ff0099" color="#fff">
              Get Started For Free
            </Button>
          </div>
          <Image src="../../../public/images/illustration-mockups.svg" alt="" />
        </Flex>
      </Container>
    </StyledHeader>
  );
};

export default Header;
