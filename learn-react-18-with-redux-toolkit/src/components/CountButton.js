import React from 'react';
import { StyledButton } from '../App';

// CountButton component
export const CountButton = ({ onClick, count }) => {
  const handleClick = () => {
    onClick();
  };

  return <StyledButton onClick={handleClick}>Count: {count}</StyledButton>;
};
