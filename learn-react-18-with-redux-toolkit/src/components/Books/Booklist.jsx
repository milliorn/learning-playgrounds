import React from "react";
import styled from "styled-components";
import { books } from "../../data/books";
import { Book } from "./Book";

// Wrapper styled component
const Wrapper = styled.article`
  display: grid; // Using CSS grid for layout
  gap: 2rem; // Gap between grid items
  margin: 5rem auto; // Margin for centering the component
  max-width: 1170px; // Maximum width for the component
  width: 90vw; // Width of the component relative to the viewport
`;

// H2 styled component
const H2 = styled.h2`
  color: white; // Text color
  font-size: 2rem; // Font size
  text-align: center; // Text alignment
  text-transform: capitalize; // Capitalize the text
`;

// BookListWrapper styled component
const BookListWrapper = styled.div`
  &.booklist {
    grid-template-columns: repeat(
      3,
      1fr
    ); // Define grid columns with equal width
    align-items: start; // Align items at the start of each column
  }
`;

// Booklist component
const Booklist = () => {
  return (
    <Wrapper>
      <H2>amazon best sellers</H2> {/* Heading for the book list */}
      <BookListWrapper className="booklist">
        {" "}
        {/* Applying the "booklist" class to BookListWrapper */}
        {books.map((book, index) => {
          return <Book key={book.id} {...book} number={index}></Book>; // Render Book component for each book item
        })}
      </BookListWrapper>
    </Wrapper>
  );
};

export default Booklist;
