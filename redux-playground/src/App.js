import React, { useState } from 'react';
import styled from 'styled-components';

import { CountButton } from './components/CountButton';

import Booklist from './components/Books/Booklist';
import MultipleReturnsFetchData from './components/Conditional-Rendering/MultipleReturnsFetchData';
import Navbar from './components/ContextAPI/Navbar';
import FetchData from './components/CustomHooks/FetchData';
import ToggleExample from './components/CustomHooks/ToggleExample';
import Events from './components/Events';
import ControlledInputs from './components/Inputs/ControlledInputs';
import MultipleInputs from './components/Inputs/MultipleInputs';
import OtherInputs from './components/Inputs/OtherInputs';
import UncontrolledInputs from './components/Inputs/UncontrolledInputs';
import ReducerBasics from './components/Reducer/useReducer';
import UseRefBasics from './components/UseRefBasics';

// StyledDiv styled component
const StyledDiv = styled.div`
  align-items: center;
  color: white;
  display: flex;
  flex-direction: column;
  font-size: 1.25rem;
  font-weight: 700;
  justify-content: center;
  text-align: center;
`;

// StyledButton component
export const StyledButton = styled.button`
  background-color: #61dafb;
  border-radius: 4px;
  border: none;
  color: #282c34;
  cursor: pointer;
  font-size: 1.25rem;
  font-weight: 700;
  margin: 1rem;
  padding: 0.5rem 1rem;
  transition: background-color 0.3s ease;
  width: 200px;

  &:hover {
    background-color: #282c34;
    border: 2px solid #61dafb;
    color: #61dafb;
  }
`;

// App component
function App() {
  const [ count, setCount ] = useState(0);

  const increment = () => {
    setCount(count + 1);
  };

  return (
    <div>
      <Navbar />
      <StyledDiv className="App">
        <MultipleReturnsFetchData />
        <FetchData />
        <ToggleExample />
        <ReducerBasics />
        <UseRefBasics />
        <UncontrolledInputs />
        <OtherInputs />
        <MultipleInputs />
        <ControlledInputs />
        <h1>React Hooks useState Counter</h1>
        <CountButton onClick={increment} count={count} />
        <Events />
      </StyledDiv>
      <Booklist />
    </div>
  );
}

export default App;
