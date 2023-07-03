import React from "react";
import styled from "styled-components";

// Wrapper styled component
const Wrapper = styled.article`
  background-color: gray; // Background color of the wrapper
  border-radius: 1rem; // Border radius for rounded corners
  box-shadow: 0 5px 15px rgba(0, 0, 0, 0.1); // Box shadow for a subtle elevation effect
  padding: 2rem; // Padding inside the wrapper
  text-align: center; // Center-align the content
  margin: 2rem; // Margin around the wrapper
`;

// Image styled component
const Image = styled.img`
  object-fit: scale-down; // Scale down the image to fit within its container
  width: 100%; // Set the width of the image to 100% of its container
`;

// H2 styled component
const H2 = styled.h2`
  color: white; // Text color
  font-size: 1.25rem; // Font size
  font-weight: bold; // Font weight
  margin-top: 1.75rem; // Margin top
`;

// H4 styled component
const H4 = styled.h4`
  color: white; // Text color
  font-size: 0.75rem; // Font size
  font-weight: bold; // Font weight
  letter-spacing: 2px; // Letter spacing
  margin-top: 0.5rem; // Margin top
`;

export const Book = (props) => {
  const { img, title, author, number } = props;

  return (
    <Wrapper>
      <Image src={img} alt={title} /> {/* Display the book cover image */}
      <H2>{title}</H2> {/* Display the book title */}
      <H4>{author}</H4> {/* Display the book author */}
      <H2>Ranked #{number + 1}</H2> {/* Display the book rank */}
    </Wrapper>
  );
};
